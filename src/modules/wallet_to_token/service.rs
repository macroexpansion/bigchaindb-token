use std::sync::Arc;

use diesel::prelude::*;
use diesel_async::RunQueryDsl;

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

    pub async fn create_wallet_to_token(
        &self,
        wallet_id: i32,
        token_id: i32,
        amount: i32,
    ) -> anyhow::Result<WalletToken> {
        use crate::database::schema::wallets_tokens::dsl;

        let mut conn = self.pool.get().await.unwrap();

        let object = dsl::wallets_tokens
            .filter(WalletToken::with_wallet_id(wallet_id))
            .filter(WalletToken::with_token_id(token_id))
            .select(WalletToken::as_select())
            .first(&mut conn)
            .await;

        match object.ok() {
            Some(object) => {
                let result = diesel::update(&object)
                    .set(UpdateWalletToken {
                        volume: object.volume + amount,
                    })
                    .returning(WalletToken::as_returning())
                    .get_result(&mut conn)
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
                    .get_result(&mut conn)
                    .await?;
                Ok(result)
            }
        }
    }
}
