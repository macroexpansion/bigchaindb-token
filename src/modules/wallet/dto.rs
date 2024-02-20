use serde::{Deserialize, Serialize};
use serde_json::Value;
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
#[schema(example = json!({ "edge_id": "42", "asset": { "name": "Devr Token" } }))]
pub struct ProvisionEdgeWallet {
    pub edge_id: String,
    #[schema(value_type = Object)]
    pub asset: Value,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct TransferToken {
    #[schema(example = "42")]
    pub edge_id: String,
}
