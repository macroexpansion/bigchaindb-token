use utoipa::OpenApi;

use crate::modules::{token, wallet};

#[derive(OpenApi)]
#[openapi(
    paths(
        wallet::get_wallet_by_id,
        wallet::provision_edge_wallet,
        wallet::transfer_token,
        token::get_token_asset,
    ),
    components(
        schemas(
            wallet::dto::ProvisionEdgeWallet,
            wallet::dto::EdgeWallet,
            wallet::dto::TransferToken,
            wallet::model::Wallet,
            token::model::Token,
            token::dto::TokenAsset,
        )
    ),
    tags(
        (name = "BigchainDB Token", description = "BigchainDB Token")
    )
)]
pub struct ApiDoc;
