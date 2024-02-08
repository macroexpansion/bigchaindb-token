use std::sync::Arc;

use bigchaindb::{
    connection::Connection,
    json::{json, Value},
    transaction::Transaction,
};
use diesel::prelude::*;
use diesel_async::RunQueryDsl;

use crate::{
    config::Config,
    database::{schema::tokens, DbPool},
    modules::wallet::model::Wallet,
};

use super::model::{NewToken, Token};

pub struct TokenService {
    config: Arc<Config>,
    pool: DbPool,
}

impl TokenService {
    pub fn new(config: Arc<Config>, pool: DbPool) -> Self {
        tracing::info!("initialized");

        Self { config, pool }
    }

    async fn create_token_database(&self, token: &str) -> anyhow::Result<Token> {
        let mut conn = self.pool.get().await.unwrap();

        let token = diesel::insert_into(tokens::table)
            .values(NewToken {
                token: token.to_owned(),
            })
            .returning(Token::as_returning())
            .get_result(&mut conn)
            .await?;

        Ok(token)
    }

    pub async fn get_token(&self, token: &str) -> anyhow::Result<Token> {
        use crate::database::schema::tokens::dsl;

        let mut conn = self.pool.get().await.unwrap();

        let token = dsl::tokens
            .filter(Token::with_token(token))
            .select(Token::as_select())
            .first(&mut conn)
            .await?;

        Ok(token)
    }

    pub async fn create_token(
        &self,
        signer: &Wallet,
        init_amount: i32,
        asset: Option<Value>,
        metadata: Option<Value>,
    ) -> anyhow::Result<Token> {
        let condition = Transaction::make_ed25519_condition(&signer.public_key, true).unwrap();
        let output = Transaction::make_output(condition, init_amount.to_string());
        let tx = Transaction::make_create_transaction(
            asset,
            metadata,
            vec![output],
            vec![signer.private_key.to_string()],
        );
        let signed_tx = Transaction::sign_transaction(&tx, vec![&signer.private_key]);

        let mut conn = Connection::new(vec![&self.config.bigchain]);

        let tx = conn.post_transaction_commit(signed_tx).await?;

        let new_token = self.create_token_database(&tx.id.clone().unwrap()).await?;
        Ok(new_token)
    }
}
