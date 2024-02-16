use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::modules::{token::model::Token, wallet};

#[derive(Serialize, Debug, ToSchema)]
pub struct Wallet {
    pub volume: i32,

    #[serde(flatten)]
    pub keypair: wallet::model::Wallet,
}

#[derive(Serialize, Debug, ToSchema)]
pub struct EdgeWallet {
    pub edge_id: i32,
    pub src_wallet: Wallet,
    pub dst_wallet: Wallet,

    #[serde(flatten)]
    pub token: Token,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct ProvisionEdgeWallet {
    pub edge_id: String,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct TransferToken {
    pub edge_id: String,
}
