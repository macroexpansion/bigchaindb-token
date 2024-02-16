use std::sync::Arc;

use axum::{
    extract::{Path, State},
    Json,
};

use crate::{error::AppError, state::AppState};

use super::dto::TokenAsset;

#[utoipa::path(
    get,
    path = "/tokens/{token}",
    tag = "tokens",
    params(
        ("token" = String, Path, description = "Token Address"),
    ),
    responses(
        (status = 200, description = "success response", body = TokenAsset)
    )
)]
pub async fn get_token_asset(
    State(state): State<Arc<AppState>>,
    Path(token): Path<String>,
) -> Result<Json<TokenAsset>, AppError> {
    let result = state
        .service
        .token
        .get_token_asset_bigchaindb(&token)
        .await?;
    Ok(Json(result))
}
