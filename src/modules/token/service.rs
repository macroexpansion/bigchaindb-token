use std::sync::Arc;

use anyhow::anyhow;
use bigchaindb::{
    connection::Connection,
    json::{json, Value},
    transaction::{Operation, Transaction, UnspentOutput},
};

use diesel::prelude::*;
use diesel_async::{pooled_connection::bb8::PooledConnection, AsyncPgConnection, RunQueryDsl};

use crate::{
    config::Config,
    database::{schema::tokens, DbPool},
    modules::wallet::model::Wallet,
};

use super::{
    dto,
    model::{NewToken, Token},
};

pub struct TokenService {
    config: Arc<Config>,
    pool: DbPool,
}

impl TokenService {
    pub fn new(config: Arc<Config>, pool: DbPool) -> Self {
        tracing::info!("initialized");

        Self { config, pool }
    }

    async fn create_token_database(
        &self,
        token: &str,
        transaction: Option<&mut PooledConnection<'_, AsyncPgConnection>>,
    ) -> anyhow::Result<Token> {
        let query = diesel::insert_into(tokens::table)
            .values(NewToken {
                token: token.to_owned(),
            })
            .returning(Token::as_returning());

        if let Some(conn) = transaction {
            Ok(query.get_result(conn).await?)
        } else {
            let mut conn = self.pool.get().await.unwrap();
            Ok(query.get_result(&mut conn).await?)
        }
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
        transaction: Option<&mut PooledConnection<'_, AsyncPgConnection>>,
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

        let new_token = self
            .create_token_database(&tx.id.clone().unwrap(), transaction)
            .await?;
        Ok(new_token)
    }

    pub async fn transfer_token(
        &self,
        sender: &Wallet,
        receiver: &Wallet,
        token: &str,
        transfer_amount: i32,
    ) -> anyhow::Result<()> {
        let mut conn = Connection::new(vec![&self.config.bigchain]);

        let list_outputs = conn.list_outputs(&sender.public_key, Some(false)).await?;

        // find unspent_output of sender's pubkey and token
        let mut unspent_outputs = Vec::new();
        for output in list_outputs.iter() {
            let tx = conn.get_transaction(&output.transaction_id).await?;
            unspent_outputs.push(UnspentOutput {
                tx,
                output_index: output.output_index,
            })
        }
        let unspent_output = unspent_outputs.iter().find(|e| match &e.tx.operation {
            Some(Operation::CREATE) => {
                if let Some(id) = &e.tx.id {
                    return id == token;
                }
                false
            }
            Some(Operation::TRANSFER) => {
                if let Some(asset) = &e.tx.asset {
                    if let Some(id) = asset.get_link_id() {
                        return id == token;
                    }
                }
                false
            }
            None => false,
        });
        let unspent_output = unspent_output.unwrap();

        let total_amount = unspent_output.tx.outputs[unspent_output.output_index]
            .amount
            .parse::<i32>()?;

        // create transaction output
        let mut outputs = Vec::new();
        for (amount, pubkey) in [
            (total_amount - transfer_amount, &sender.public_key),
            (transfer_amount, &receiver.public_key),
        ] {
            if amount > 0 {
                outputs.push(Transaction::make_output(
                    Transaction::make_ed25519_condition(pubkey, true).unwrap(),
                    amount.to_string(),
                ));
            }
        }

        // make transfer transaction
        let transfer_tx = Transaction::make_transfer_transaction(
            vec![unspent_output.clone()],
            outputs,
            Some(json!({
                "transfer_to": &receiver.public_key,
                "transfer_amount": transfer_amount,
            })),
        );

        // signed tranasction with sender's private_key
        let signed_tx = Transaction::sign_transaction(&transfer_tx, vec![&sender.private_key]);

        // commit tranasction to BigchainDB
        let _tx = conn.post_transaction_commit(signed_tx).await?;

        Ok(())
    }

    pub async fn get_token_asset_bigchaindb(&self, token: &str) -> anyhow::Result<dto::TokenAsset> {
        let mut conn = Connection::new(vec![&self.config.bigchain]);
        let tx = conn.get_transaction(token).await?;
        let asset = tx.asset.ok_or(anyhow!("token has no asset"))?;
        let asset = asset
            .get_definition_data()
            .ok_or(anyhow!("token has no CREATE asset"))?;

        Ok(dto::TokenAsset {
            asset: asset.clone(),
        })
    }
}
