use std::sync::Arc;

use bigchaindb::{ed25519_keypair, json::json};
use diesel::prelude::*;
use diesel_async::{
    pooled_connection::bb8::PooledConnection, scoped_futures::ScopedFutureExt, AsyncConnection,
    AsyncPgConnection, RunQueryDsl,
};

use crate::{
    config::Config,
    database::{
        schema::{tokens, wallets},
        DbPool,
    },
    modules::{
        edge::EdgeService,
        token::{model::Token, TokenService},
        wallet_to_token::{model::WalletToken, WalletTokenService},
    },
};

use super::{
    dto,
    model::{NewWallet, Wallet},
};

pub struct WalletService {
    config: Arc<Config>,
    pool: DbPool,
    token_service: Arc<TokenService>,
    wallet_to_token_service: Arc<WalletTokenService>,
    edge_service: Arc<EdgeService>,
}

impl WalletService {
    const INIT_AMOUNT: i32 = 100;

    pub fn new(
        config: Arc<Config>,
        pool: DbPool,
        token_service: Arc<TokenService>,
        wallet_to_token_service: Arc<WalletTokenService>,
        edge_service: Arc<EdgeService>,
    ) -> Self {
        tracing::info!("initialized");

        Self {
            config,
            pool,
            token_service,
            wallet_to_token_service,
            edge_service,
        }
    }

    pub async fn get_wallet_by_id(&self, id: i32) -> anyhow::Result<Wallet> {
        use crate::database::schema::wallets::dsl;

        let mut conn = self.pool.get().await.unwrap();

        let wallet = dsl::wallets
            .filter(Wallet::with_id(id))
            .select(Wallet::as_select())
            .first(&mut conn)
            .await?;

        Ok(wallet)
    }

    pub async fn create_wallet(
        &self,
        transaction: Option<&mut PooledConnection<'_, AsyncPgConnection>>,
    ) -> anyhow::Result<Wallet> {
        let keypair = ed25519_keypair();

        let query = diesel::insert_into(wallets::table)
            .values(NewWallet {
                public_key: keypair.pk.clone(),
                private_key: keypair.sk.clone(),
            })
            .returning(Wallet::as_returning());

        let wallet = if let Some(conn) = transaction {
            query.get_result(conn).await?
        } else {
            let mut conn = self.pool.get().await.unwrap();
            query.get_result(&mut conn).await?
        };

        Ok(wallet)
    }

    pub async fn get_edge_wallet(&self, edge_id: i32) -> anyhow::Result<dto::EdgeWallet> {
        let edge_wallet = self.edge_service.get_edge_to_wallet(edge_id).await?;
        let src_wallet = self.get_wallet_by_id(edge_wallet.src_wallet_id).await?;
        let dst_wallet = self.get_wallet_by_id(edge_wallet.dst_wallet_id).await?;

        self.populate_edge_wallet(edge_id, src_wallet, dst_wallet)
            .await
    }

    pub async fn populate_edge_wallet(
        &self,
        edge_id: i32,
        src_wallet: Wallet,
        dst_wallet: Wallet,
    ) -> anyhow::Result<dto::EdgeWallet> {
        let mut conn = self.pool.get().await.unwrap();

        let src_wallet_token = WalletToken::belonging_to(&src_wallet)
            .inner_join(tokens::table)
            .select((WalletToken::as_select(), Token::as_select()))
            .first::<(WalletToken, Token)>(&mut conn)
            .await?;
        let dst_wallet_token = WalletToken::belonging_to(&dst_wallet)
            .inner_join(tokens::table)
            .select(WalletToken::as_select())
            .first::<WalletToken>(&mut conn)
            .await?;

        let src_wallet = dto::Wallet {
            keypair: src_wallet,
            volume: src_wallet_token.0.volume,
        };
        let dst_wallet = dto::Wallet {
            keypair: dst_wallet,
            volume: dst_wallet_token.volume,
        };

        return Ok(dto::EdgeWallet {
            edge_id,
            src_wallet,
            dst_wallet,
            token: src_wallet_token.1,
        });
    }

    pub async fn provision_edge_wallet(&self, edge_id: i32) -> anyhow::Result<dto::EdgeWallet> {
        let edge_wallet = self.get_edge_wallet(edge_id).await;
        if edge_wallet.is_ok() {
            return edge_wallet;
        }

        let mut conn = self.pool.get().await.unwrap();
        conn.transaction::<_, diesel::result::Error, _>(|conn| {
            async move {
                let src_wallet = self.create_wallet(Some(conn)).await.map_err(|e| {
                    println!("create_dst_wallet: {e:?}");
                    diesel::result::Error::RollbackTransaction
                })?;
                let dst_wallet = self.create_wallet(Some(conn)).await.map_err(|e| {
                    println!("create_dst_wallet: {e:?}");
                    diesel::result::Error::RollbackTransaction
                })?;

                let asset = json!({
                    "token": "Devr Token",
                    "num_tokens": Self::INIT_AMOUNT,
                });
                let metadata = json!({ "co": "devr" });
                let token = self
                    .token_service
                    .create_token(&src_wallet, 100, Some(asset), Some(metadata), Some(conn))
                    .await
                    .map_err(|e| {
                        println!("create_token: {e:?}");
                        diesel::result::Error::RollbackTransaction
                    })?;

                let _ = self
                    .wallet_to_token_service
                    .upsert_wallet_token(src_wallet.id, token.id, Self::INIT_AMOUNT, Some(conn))
                    .await
                    .map_err(|e| {
                        println!("upsert src wallet token: {e:?}");
                        diesel::result::Error::RollbackTransaction
                    })?;
                let _ = self
                    .wallet_to_token_service
                    .upsert_wallet_token(dst_wallet.id, token.id, 0, Some(conn))
                    .await
                    .map_err(|e| {
                        println!("upsert dst wallet token: {e:?}");
                        diesel::result::Error::RollbackTransaction
                    })?;

                let _ = self
                    .edge_service
                    .create_edge_to_wallet(edge_id, src_wallet.id, dst_wallet.id, Some(conn))
                    .await
                    .map_err(|e| {
                        println!("create edge to wallet: {e:?}");
                        diesel::result::Error::RollbackTransaction
                    })?;

                Ok(())
            }
            .scope_boxed()
        })
        .await?;

        self.get_edge_wallet(edge_id).await
    }

    pub async fn transfer_token(&self, edge_id: i32) -> anyhow::Result<dto::EdgeWallet> {
        let edge_wallet = self.get_edge_wallet(edge_id).await?;

        let _ = self
            .token_service
            .transfer_token(
                &edge_wallet.src_wallet.keypair,
                &edge_wallet.dst_wallet.keypair,
                &edge_wallet.token.token,
                1,
            )
            .await?;

        let mut conn = self.pool.get().await.unwrap();
        conn.transaction::<_, diesel::result::Error, _>(|conn| {
            async move {
                let _ = self
                    .wallet_to_token_service
                    .upsert_wallet_token(
                        edge_wallet.src_wallet.keypair.id,
                        edge_wallet.token.id,
                        -1,
                        Some(conn),
                    )
                    .await
                    .map_err(|_| diesel::result::Error::RollbackTransaction)?;

                let _ = self
                    .wallet_to_token_service
                    .upsert_wallet_token(
                        edge_wallet.dst_wallet.keypair.id,
                        edge_wallet.token.id,
                        1,
                        Some(conn),
                    )
                    .await
                    .map_err(|_| diesel::result::Error::RollbackTransaction)?;

                Ok(())
            }
            .scope_boxed()
        })
        .await?;

        self.get_edge_wallet(edge_id).await
    }
}
