use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::modules::wallet::model;

#[derive(Serialize, Debug, ToSchema)]
pub struct Wallet {
    #[serde(flatten)]
    pub keypair: model::Wallet,
    pub volume: i32,
}

#[derive(Serialize, Debug, ToSchema)]
pub struct EdgeWallet {
    pub edge_id: i32,
    pub src_wallet: Wallet,
    pub dst_wallet: Wallet,
    pub token: String,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct ProvisionEdgeWallet {
    pub edge_id: String,
}
