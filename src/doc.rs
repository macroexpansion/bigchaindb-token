use utoipa::OpenApi;

use crate::modules::{token, wallet};

#[derive(OpenApi)]
#[openapi(
    paths(
        wallet::get_wallet_by_id,
        wallet::provision_edge_wallet,
    ),
    components(
        schemas(
            wallet::dto::ProvisionEdgeWallet,
            wallet::dto::EdgeWallet,
            wallet::model::Wallet,
            token::model::Token,
        )
    ),
    tags(
        (name = "BigchainDB Token", description = "BigchainDB Token")
    )
)]
pub struct ApiDoc;
