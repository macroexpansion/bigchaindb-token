fn main() {}

// use bigchaindb::{
//     connection::Connection,
//     json::json,
//     transaction::TransactionTemplate,
//     transaction::{Operation, Output, Transaction, UnspentOutput},
//     Ed25519Keypair,
// };
// use diesel::prelude::*;
// use diesel_async::RunQueryDsl;
// use dotenv::dotenv;
// use serde::{Deserialize, Serialize};
// use utoipa::ToSchema;
//
// use bigchaindb_token::database::{
//     schema::{tokens, wallets, wallets_tokens},
//     DatabaseConnPool, DbPool,
// };
//
// #[derive(Debug, Serialize, Selectable, Queryable, ToSchema)]
// #[diesel(table_name = wallets)]
// #[diesel(check_for_backend(diesel::pg::Pg))]
// pub struct Wallet {
//     pub id: i32,
//     pub device_id: i32,
//     pub public_key: String,
//     pub private_key: String,
// }
//
// type WithPubkey<'a> = diesel::dsl::Eq<wallets::public_key, &'a str>;
//
// impl Wallet {
//     pub fn with_pubkey(pubkey: &str) -> WithPubkey {
//         wallets::public_key.eq(pubkey)
//     }
// }
//
// #[derive(Deserialize, Insertable, ToSchema)]
// #[diesel(table_name = wallets)]
// #[diesel(check_for_backend(diesel::pg::Pg))]
// pub struct NewWallet {
//     pub device_id: i32,
//     pub public_key: String,
//     pub private_key: String,
// }
//
// #[derive(Debug, Serialize, Selectable, Queryable, ToSchema)]
// #[diesel(table_name = tokens)]
// #[diesel(check_for_backend(diesel::pg::Pg))]
// pub struct Token {
//     pub id: i32,
//     pub token: String,
// }
//
// type WithToken<'a> = diesel::dsl::Eq<tokens::token, &'a str>;
//
// impl Token {
//     pub fn with_token(token: &str) -> WithToken {
//         tokens::token.eq(token)
//     }
// }
//
// #[derive(Deserialize, Insertable, ToSchema)]
// #[diesel(table_name = tokens)]
// #[diesel(check_for_backend(diesel::pg::Pg))]
// pub struct NewToken {
//     pub token: String,
// }
//
// #[derive(Debug, Serialize, Identifiable, Selectable, Queryable, ToSchema)]
// #[diesel(table_name = wallets_tokens)]
// #[diesel(check_for_backend(diesel::pg::Pg))]
// pub struct WalletToToken {
//     pub id: i32,
//     pub wallet_id: i32,
//     pub token_id: i32,
//     pub volume: i32,
// }
//
// type WithWalletId = diesel::dsl::Eq<wallets_tokens::wallet_id, i32>;
// type WithTokenId = diesel::dsl::Eq<wallets_tokens::token_id, i32>;
//
// impl WalletToToken {
//     pub fn with_token_id(id: i32) -> WithTokenId {
//         wallets_tokens::token_id.eq(id)
//     }
//
//     pub fn with_wallet_id(id: i32) -> WithWalletId {
//         wallets_tokens::wallet_id.eq(id)
//     }
// }
//
// #[derive(Deserialize, Insertable, ToSchema)]
// #[diesel(table_name = wallets_tokens)]
// #[diesel(check_for_backend(diesel::pg::Pg))]
// pub struct NewWalletToToken {
//     pub wallet_id: i32,
//     pub token_id: i32,
//     pub volume: i32,
// }
//
// #[derive(AsChangeset)]
// #[diesel(table_name = wallets_tokens)]
// #[diesel(check_for_backend(diesel::pg::Pg))]
// pub struct UpdateWalletToToken {
//     pub volume: i32,
// }
//
// #[tokio::main]
// async fn main() {
//     // setup
//     dotenv().ok();
//     let db_url = std::env::var("DATABASE_URL").unwrap();
//     let bigchain_url =
//         std::env::var("BIGCHAINDB_URL").unwrap_or("http://localhost:9984/api/v1/".to_string());
//     let pool = DatabaseConnPool::new(&db_url).await;
//
//     // create keypair
//     let signer = Ed25519Keypair {
//         pk: "8Humby652oBRwtEu4HqVQfJ5UzfB5hd4q2V8HMMDuTBJ".to_string(),
//         sk: "5TzJ85b5NaXdt8msvkFztoJnpUgAgwHar42LXtY3GyWM".to_string(),
//     };
//     let user_1 = Ed25519Keypair {
//         pk: "8ECWYLRcfabcZVRTHfsfWeVLTUt2YmjZuHQBhkbdXcKX".to_string(),
//         sk: "99TEbdqYroRs2nQLbpzH9eLVtNMVXqxM2i4xwfafnfoM".to_string(),
//     };
//     let user_2 = Ed25519Keypair {
//         pk: "9GKuyXjY1cAHJN6Uh5JtLzR3C18jJqk43XtKHcGtTmAH".to_string(),
//         sk: "AAm3zjzJG3VFGrzSXEmyp7RWF4caMErKjcabxSaLRYvz".to_string(),
//     };
//
//     // create wallet
//     let source = get_or_create_wallet(pool.clone(), &signer).await;
//     // println!("{signer_wallet:?}");
//     let dest = get_or_create_wallet(pool.clone(), &user_1).await;
//     // let user_wallet_2 = get_or_create_wallet(pool.clone(), &user_2).await;
//
//     // let tx = create_token_transaction(&signer, &bigchain_url)
//     //     .await
//     //     .unwrap();
//     // let token = get_or_create_token(pool.clone(), Some(&tx.id.unwrap())).await;
//     // let wallet_token = upsert_wallet_to_token(pool.clone(), signer_wallet.id, token.id, 100)
//     //     .await
//     //     .unwrap();
//     // println!("{wallet_token:?}");
//
//     // let token = get_or_create_token(pool.clone(), None).await;
//     // println!("{token:?}");
//     // let _ = transfer_token(&source, &dest, &token.token, 1, &bigchain_url)
//     //     .await
//     //     .unwrap();
//     // let update_sender = upsert_wallet_to_token(pool.clone(), signer_wallet.id, token.id, -1)
//     //     .await
//     //     .unwrap();
//     // println!("{update_sender:?}");
//     // let update_receiver = upsert_wallet_to_token(pool.clone(), user_wallet_1.id, token.id, 1)
//     //     .await
//     //     .unwrap();
//     // println!("{update_receiver:?}");
//
//     let mut conn = Connection::new(vec![&bigchain_url]);
//     let tx = conn
//         .get_transaction("c0635696a0d2f4ab88379769114096200765872ebe6808796799e787c68cbf5b")
//         .await
//         .unwrap();
//     println!("{tx:#?}");
// }
//
// async fn upsert_wallet_to_token(
//     pool: DbPool,
//     wallet_id: i32,
//     token_id: i32,
//     amount: i32,
// ) -> anyhow::Result<WalletToToken> {
//     use bigchaindb_token::database::schema::wallets_tokens::dsl;
//
//     let mut conn = pool.get().await.unwrap();
//
//     let object = dsl::wallets_tokens
//         .filter(WalletToToken::with_wallet_id(wallet_id))
//         .filter(WalletToToken::with_token_id(token_id))
//         .select(WalletToToken::as_select())
//         .first(&mut conn)
//         .await;
//
//     match object.ok() {
//         Some(object) => {
//             let result = diesel::update(&object)
//                 .set(UpdateWalletToToken {
//                     volume: object.volume + amount,
//                 })
//                 .returning(WalletToToken::as_returning())
//                 .get_result(&mut conn)
//                 .await?;
//             Ok(result)
//         }
//         None => {
//             let result = diesel::insert_into(wallets_tokens::table)
//                 .values(NewWalletToToken {
//                     wallet_id,
//                     token_id,
//                     volume: amount,
//                 })
//                 .returning(WalletToToken::as_returning())
//                 .get_result(&mut conn)
//                 .await?;
//             Ok(result)
//         }
//     }
// }
//
// async fn transfer_token(
//     sender: &Wallet,
//     receiver: &Wallet,
//     token: &str,
//     transfer_amount: i32,
//     bigchain_url: &str,
// ) -> anyhow::Result<()> {
//     let mut conn = Connection::new(vec![&bigchain_url]);
//
//     let list_outputs = conn
//         .list_outputs(&sender.public_key, Some(false))
//         .await
//         .unwrap();
//
//     // find unspent_output of sender's pubkey and token
//     let mut unspent_outputs = Vec::new();
//     for output in list_outputs.iter() {
//         let tx = conn.get_transaction(&output.transaction_id).await.unwrap();
//         unspent_outputs.push(UnspentOutput {
//             tx,
//             output_index: output.output_index,
//         })
//     }
//     let unspent_output = unspent_outputs.iter().find(|e| match &e.tx.operation {
//         Some(Operation::CREATE) => {
//             if let Some(id) = &e.tx.id {
//                 return id == token;
//             }
//             false
//         }
//         Some(Operation::TRANSFER) => {
//             if let Some(asset) = &e.tx.asset {
//                 if let Some(id) = asset.get_link_id() {
//                     return id == token;
//                 }
//             }
//             false
//         }
//         None => false,
//     });
//     let unspent_output = unspent_output.unwrap();
//
//     let total_amount = unspent_output.tx.outputs[unspent_output.output_index]
//         .amount
//         .parse::<i32>()
//         .unwrap();
//
//     // create transaction output
//     let mut outputs = Vec::new();
//     for (amount, pubkey) in [
//         (total_amount - transfer_amount, &sender.public_key),
//         (transfer_amount, &receiver.public_key),
//     ] {
//         if amount > 0 {
//             outputs.push(Transaction::make_output(
//                 Transaction::make_ed25519_condition(pubkey, true).unwrap(),
//                 amount.to_string(),
//             ));
//         }
//     }
//
//     // make transfer transaction
//     let transfer_tx = Transaction::make_transfer_transaction(
//         vec![unspent_output.clone()],
//         outputs,
//         Some(json!({
//             "transfer_to": &receiver.public_key,
//             "transfer_amount": transfer_amount,
//         })),
//     );
//
//     // signed tranasction with sender's private_key
//     let signed_tx = Transaction::sign_transaction(&transfer_tx, vec![&sender.private_key]);
//     println!("signed_tx {signed_tx:?}");
//
//     // commit tranasction to BigchainDB
//     let tx = conn.post_transaction_commit(signed_tx).await.unwrap();
//     println!("{tx:?}");
//
//     Ok(())
// }
//
// async fn get_or_create_token(pool: DbPool, token: Option<&str>) -> Token {
//     use bigchaindb_token::database::schema::tokens::dsl;
//
//     let mut conn = pool.get().await.unwrap();
//
//     if let Some(token) = token {
//         let result = diesel::insert_into(tokens::table)
//             .values(NewToken {
//                 token: token.to_owned(),
//             })
//             .returning(Token::as_returning())
//             .get_result(&mut conn)
//             .await
//             .unwrap();
//         return result;
//     }
//
//     let token = dsl::tokens
//         .select(Token::as_select())
//         .first(&mut conn)
//         .await
//         .unwrap();
//
//     token
// }
//
// async fn get_or_create_wallet_to_token(
//     pool: DbPool,
//     wallet_id: i32,
//     token_id: i32,
// ) -> WalletToToken {
//     use bigchaindb_token::database::schema::wallets_tokens::dsl;
//
//     let mut conn = pool.get().await.unwrap();
//
//     let result = dsl::wallets_tokens
//         .filter(WalletToToken::with_wallet_id(wallet_id))
//         .filter(WalletToToken::with_token_id(token_id))
//         .select(WalletToToken::as_select())
//         .first(&mut conn)
//         .await;
//     if let Some(result) = result.ok() {
//         return result;
//     }
//
//     let result = diesel::insert_into(wallets_tokens::table)
//         .values(NewWalletToToken {
//             wallet_id,
//             token_id,
//             volume: 0,
//         })
//         .returning(WalletToToken::as_returning())
//         .get_result(&mut conn)
//         .await
//         .unwrap();
//
//     result
// }
//
// async fn get_or_create_wallet(pool: DbPool, keypair: &Ed25519Keypair) -> Wallet {
//     let mut conn = pool.get().await.unwrap();
//
//     use bigchaindb_token::database::schema::wallets::dsl;
//
//     let wallet = dsl::wallets
//         .filter(Wallet::with_pubkey(&keypair.pk))
//         .select(Wallet::as_select())
//         .first(&mut conn)
//         .await;
//     if let Some(wallet) = wallet.ok() {
//         return wallet;
//     }
//
//     let result = diesel::insert_into(wallets::table)
//         .values(NewWallet {
//             device_id: 0,
//             public_key: keypair.pk.clone(),
//             private_key: keypair.sk.clone(),
//         })
//         .returning(Wallet::as_returning())
//         .get_result(&mut conn)
//         .await
//         .unwrap();
//     result
// }
//
// async fn create_token_transaction(
//     signer: &Ed25519Keypair,
//     bigchain_url: &str,
// ) -> anyhow::Result<TransactionTemplate> {
//     let num_tokens = "100";
//     let asset = json!({
//         "token": "Devr Token",
//         "num_tokens": num_tokens,
//     });
//     let metadata = json!({ "co": "devr" });
//
//     let condition = Transaction::make_ed25519_condition(&signer.pk, true).unwrap();
//     let output = Transaction::make_output(condition, String::from(num_tokens));
//     let tx = Transaction::make_create_transaction(
//         Some(asset),
//         Some(metadata),
//         vec![output],
//         vec![signer.pk.to_string()],
//     );
//     let signed_tx = Transaction::sign_transaction(&tx, vec![&signer.sk]);
//
//     let mut conn = Connection::new(vec![&bigchain_url]);
//
//     Ok(conn.post_transaction_commit(signed_tx).await?)
// }
