use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::database::schema::edges_wallets;

#[derive(Debug, Serialize, Identifiable, Selectable, Queryable, ToSchema)]
#[diesel(table_name = edges_wallets)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct EdgeWallet {
    #[serde(skip_serializing)]
    pub id: i32,
    pub edge_id: i32,
    pub src_wallet_id: i32,
    pub dst_wallet_id: i32,
}

type WithEdgeId = diesel::dsl::Eq<edges_wallets::edge_id, i32>;
type WithSrcWalletId = diesel::dsl::Eq<edges_wallets::src_wallet_id, i32>;

impl EdgeWallet {
    pub fn with_edge_id(id: i32) -> WithEdgeId {
        edges_wallets::edge_id.eq(id)
    }

    pub fn with_src_wallet_id(id: i32) -> WithSrcWalletId {
        edges_wallets::src_wallet_id.eq(id)
    }
}

#[derive(Deserialize, Insertable, ToSchema)]
#[diesel(table_name = edges_wallets)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewEdgeWallet {
    pub edge_id: i32,
    pub src_wallet_id: i32,
    pub dst_wallet_id: i32,
}
