use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::{
    database::schema::wallets_tokens,
    modules::{token::model::Token, wallet::model::Wallet},
};

#[derive(Debug, Serialize, Selectable, Identifiable, Queryable, Associations, ToSchema)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(primary_key(wallet_id, token_id))]
#[diesel(table_name = wallets_tokens)]
#[diesel(belongs_to(Wallet))]
#[diesel(belongs_to(Token))]
pub struct WalletToken {
    pub wallet_id: i32,
    pub token_id: i32,
    pub volume: i32,
}

type WithWalletId = diesel::dsl::Eq<wallets_tokens::wallet_id, i32>;
type WithTokenId = diesel::dsl::Eq<wallets_tokens::token_id, i32>;

impl WalletToken {
    pub fn with_token_id(id: i32) -> WithTokenId {
        wallets_tokens::token_id.eq(id)
    }

    pub fn with_wallet_id(id: i32) -> WithWalletId {
        wallets_tokens::wallet_id.eq(id)
    }
}

#[derive(Deserialize, Insertable, ToSchema)]
#[diesel(table_name = wallets_tokens)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewWalletToken {
    pub wallet_id: i32,
    pub token_id: i32,
    pub volume: i32,
}

#[derive(AsChangeset)]
#[diesel(table_name = wallets_tokens)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct UpdateWalletToken {
    pub volume: i32,
}
