use bigchaindb::{
    connection::Connection, json::json, transaction::Transaction, transaction::TransactionTemplate,
    Ed25519Keypair,
};
use diesel::{dsl::AsSelect, pg::Pg, prelude::*};
use diesel_async::RunQueryDsl;
use dotenv::dotenv;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use bigchaindb_token::database::{
    schema::{tokens, wallets},
    DatabaseConnPool, DbPool,
};

#[derive(Debug, Serialize, Selectable, Queryable, ToSchema)]
#[diesel(table_name = wallets)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Wallet {
    pub id: i32,
    pub public_key: String,
    pub private_key: String,
}

type WithPubkey<'a> = diesel::dsl::Eq<wallets::public_key, &'a str>;

impl Wallet {
    pub fn with_pubkey(pubkey: &str) -> WithPubkey {
        wallets::public_key.eq(pubkey)
    }
}

#[derive(Deserialize, Insertable, ToSchema)]
#[diesel(table_name = wallets)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewWallet {
    pub public_key: String,
    pub private_key: String,
}

#[derive(Debug, Serialize, Selectable, Queryable, ToSchema)]
#[diesel(table_name = tokens)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Token {
    pub id: i32,
    pub wallet_id: i32,
    pub token: String,
    pub volume: i32,
}

type WithToken<'a> = diesel::dsl::Eq<tokens::token, &'a str>;

impl Token {
    pub fn with_token(token: &str) -> WithToken {
        tokens::token.eq(token)
    }
}

#[derive(Deserialize, Insertable, ToSchema)]
#[diesel(table_name = tokens)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewToken {
    pub wallet_id: i32,
    pub token: String,
    pub volume: i32,
}

#[tokio::main]
async fn main() {
    // setup
    dotenv().ok();
    let db_url = std::env::var("DATABASE_URL").unwrap();
    let bigchain_url =
        std::env::var("BIGCHAINDB_URL").unwrap_or("http://localhost:9984/api/v1/".to_string());
    let pool = DatabaseConnPool::new(&db_url).await;

    // create keypair
    let signer = Ed25519Keypair {
        pk: "8Humby652oBRwtEu4HqVQfJ5UzfB5hd4q2V8HMMDuTBJ".to_string(),
        sk: "5TzJ85b5NaXdt8msvkFztoJnpUgAgwHar42LXtY3GyWM".to_string(),
    };
    let user_1 = Ed25519Keypair {
        pk: "8ECWYLRcfabcZVRTHfsfWeVLTUt2YmjZuHQBhkbdXcKX".to_string(),
        sk: "99TEbdqYroRs2nQLbpzH9eLVtNMVXqxM2i4xwfafnfoM".to_string(),
    };
    let user_2 = Ed25519Keypair {
        pk: "9GKuyXjY1cAHJN6Uh5JtLzR3C18jJqk43XtKHcGtTmAH".to_string(),
        sk: "AAm3zjzJG3VFGrzSXEmyp7RWF4caMErKjcabxSaLRYvz".to_string(),
    };

    // create wallet
    let signer_wallet = get_wallet(pool.clone(), &signer).await;
    println!("{signer_wallet:?}");
    let user_wallet_1 = get_wallet(pool.clone(), &user_1).await;
    let user_wallet_2 = get_wallet(pool.clone(), &user_2).await;

    let tx = create_token(&signer, &bigchain_url).await.unwrap();

    let signer_token = wallet_to_token(pool.clone(), signer_wallet.id, &tx.id.unwrap()).await;
}

async fn wallet_to_token(pool: DbPool, wallet_id: i32, token: &str) -> Token {
    let mut conn = pool.get().await.unwrap();
    let result = diesel::insert_into(tokens::table)
        .values(NewToken {
            wallet_id,
            token: token.to_owned(),
            volume: 100,
        })
        .returning(Token::as_returning())
        .get_result(&mut conn)
        .await
        .unwrap();

    println!("{result:?}");

    result
}

async fn get_wallet(pool: DbPool, keypair: &Ed25519Keypair) -> Wallet {
    let mut conn = pool.get().await.unwrap();

    use bigchaindb_token::database::schema::wallets::dsl;

    let wallet = dsl::wallets
        .filter(Wallet::with_pubkey(&keypair.pk))
        .select(Wallet::as_select())
        .first(&mut conn)
        .await;
    if let Some(wallet) = wallet.ok() {
        return wallet;
    }

    let result = diesel::insert_into(wallets::table)
        .values(NewWallet {
            public_key: keypair.pk.clone(),
            private_key: keypair.sk.clone(),
        })
        .returning(Wallet::as_returning())
        .get_result(&mut conn)
        .await
        .unwrap();
    result
}

async fn create_token(
    signer: &Ed25519Keypair,
    bigchain_url: &str,
) -> anyhow::Result<TransactionTemplate> {
    let num_tokens = "100";
    let asset = json!({
        "token": "Devr Token",
        "num_tokens": num_tokens,
    });
    let metadata = json!({ "co": "devr" });

    let condition = Transaction::make_ed25519_condition(&signer.pk, true).unwrap();
    let output = Transaction::make_output(condition, String::from(num_tokens));
    let tx = Transaction::make_create_transaction(
        Some(asset),
        metadata,
        vec![output],
        vec![signer.pk.to_string()],
    );
    let signed_tx = Transaction::sign_transaction(&tx, vec![&signer.sk]);

    let mut conn = Connection::new(vec![&bigchain_url]);

    Ok(conn.post_transaction_commit(signed_tx).await?)
}
