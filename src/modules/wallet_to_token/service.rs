use std::sync::Arc;

use diesel::prelude::*;
use diesel_async::{pooled_connection::bb8::PooledConnection, AsyncPgConnection, RunQueryDsl};

use crate::{
    config::Config,
    database::{schema::wallets_tokens, DbPool},
};

use super::model::{NewWalletToken, UpdateWalletToken, WalletToken};

pub struct WalletTokenService {
    config: Arc<Config>,
    pool: DbPool,
}

impl WalletTokenService {
    pub fn new(config: Arc<Config>, pool: DbPool) -> Self {
        tracing::info!("initialized");

        Self { config, pool }
    }

    pub async fn upsert_wallet_token(
        &self,
        wallet_id: i32,
        token_id: i32,
        amount: i32,
        transaction: Option<&mut PooledConnection<'_, AsyncPgConnection>>,
    ) -> anyhow::Result<WalletToken> {
        use crate::database::schema::wallets_tokens::dsl;

        let query = dsl::wallets_tokens
            .filter(WalletToken::with_wallet_id(wallet_id))
            .filter(WalletToken::with_token_id(token_id))
            .select(WalletToken::as_select());

        if let Some(conn) = transaction {
            let result = query.first(conn).await;
            Ok(self
                .insert_or_update(result.ok(), wallet_id, token_id, amount, conn)
                .await?)
        } else {
            let mut conn = self.pool.get().await.unwrap();
            let result = query.first(&mut conn).await;
            Ok(self
                .insert_or_update(result.ok(), wallet_id, token_id, amount, &mut conn)
                .await?)
        }
    }

    async fn insert_or_update(
        &self,
        result: Option<WalletToken>,
        wallet_id: i32,
        token_id: i32,
        amount: i32,
        conn: &mut PooledConnection<'_, AsyncPgConnection>,
    ) -> anyhow::Result<WalletToken> {
        match result {
            Some(object) => {
                let result = diesel::update(&object)
                    .set(UpdateWalletToken {
                        volume: object.volume + amount,
                    })
                    .returning(WalletToken::as_returning())
                    .get_result(conn)
                    .await?;

                Ok(result)
            }
            None => {
                let result = diesel::insert_into(wallets_tokens::table)
                    .values(NewWalletToken {
                        wallet_id,
                        token_id,
                        volume: amount,
                    })
                    .returning(WalletToken::as_returning())
                    .get_result(conn)
                    .await?;
                Ok(result)
            }
        }
    }
}
