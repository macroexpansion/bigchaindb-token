use utoipa::OpenApi;

use crate::modules::{token, wallet};

#[derive(OpenApi)]
#[openapi(
    paths(
        wallet::get_wallet_by_id,
    ),
    components(schemas(
        wallet::dto::EdgeWallet,
        wallet::model::Wallet,
        token::model::Token,
    )),
    tags(
        (name = "Token", description = "Token")
    )
)]
pub struct ApiDoc;
