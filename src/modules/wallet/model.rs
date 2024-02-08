use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::database::schema::wallets;

#[derive(Debug, Serialize, Identifiable, Selectable, Queryable, ToSchema)]
#[diesel(table_name = wallets)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Wallet {
    #[serde(skip_serializing)]
    pub id: i32,
    pub public_key: String,
    pub private_key: String,
}

type WithPubkey<'a> = diesel::dsl::Eq<wallets::public_key, &'a str>;
type WithId = diesel::dsl::Eq<wallets::id, i32>;

impl Wallet {
    pub fn with_id(id: i32) -> WithId {
        wallets::id.eq(id)
    }

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
