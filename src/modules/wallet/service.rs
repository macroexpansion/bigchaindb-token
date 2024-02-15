use std::sync::Arc;

use bigchaindb::{ed25519_keypair, json::json};
use diesel::prelude::*;
use diesel_async::RunQueryDsl;

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

    pub async fn create_wallet(&self) -> anyhow::Result<Wallet> {
        let mut conn = self.pool.get().await.unwrap();

        let keypair = ed25519_keypair();

        let wallet = diesel::insert_into(wallets::table)
            .values(NewWallet {
                public_key: keypair.pk.clone(),
                private_key: keypair.sk.clone(),
            })
            .returning(Wallet::as_returning())
            .get_result(&mut conn)
            .await?;

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
            token: src_wallet_token.1.token,
        });
    }

    pub async fn provision_edge_wallet(&self, edge_id: i32) -> anyhow::Result<dto::EdgeWallet> {
        let edge_wallet = self.edge_service.get_edge_to_wallet(edge_id).await;
        if edge_wallet.is_ok() {
            let edge_wallet = edge_wallet.unwrap();
            let src_wallet = self.get_wallet_by_id(edge_wallet.src_wallet_id).await?;
            let dst_wallet = self.get_wallet_by_id(edge_wallet.dst_wallet_id).await?;

            return self
                .populate_edge_wallet(edge_id, src_wallet, dst_wallet)
                .await;
        }

        let src_wallet = self.create_wallet().await?;
        let dst_wallet = self.create_wallet().await?;

        let asset = json!({
            "token": "Devr Token",
            "num_tokens": Self::INIT_AMOUNT,
        });
        let metadata = json!({ "co": "devr" });
        let token = self
            .token_service
            .create_token(&src_wallet, 100, Some(asset), Some(metadata))
            .await?;

        let _ = self
            .wallet_to_token_service
            .create_wallet_to_token(src_wallet.id, token.id, Self::INIT_AMOUNT)
            .await?;
        let _ = self
            .wallet_to_token_service
            .create_wallet_to_token(dst_wallet.id, token.id, 0)
            .await?;
        let _ = self
            .edge_service
            .create_edge_to_wallet(edge_id, src_wallet.id, dst_wallet.id)
            .await?;

        self.populate_edge_wallet(edge_id, src_wallet, dst_wallet)
            .await
    }
}
