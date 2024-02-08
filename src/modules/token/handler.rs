use std::sync::Arc;

use axum::{
    extract::{Path, State},
    Json,
};

use crate::{error::AppError, state::AppState};

use super::model::Token;

#[utoipa::path(
    get,
    path = "/tokens/{pubkey}",
    tag = "tokens",
    params(
        ("pubkey" = String, Path, description = "Wallet Public Key"),
    ),
    responses(
        (status = 200, description = "success response", body = Token)
    )
)]
pub async fn get_wallet_by_id(
    State(state): State<Arc<AppState>>,
    Path(device_id): Path<i32>,
) -> Result<Json<Vec<Token>>, AppError> {
    // let result = state.service.token.get_token(device_id).await?;
    // Ok(Json(result))
    todo!()
}
