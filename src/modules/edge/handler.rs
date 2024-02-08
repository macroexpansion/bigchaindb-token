use std::sync::Arc;

use axum::{
    extract::{Path, State},
    Json,
};

use crate::{error::AppError, state::AppState};

use super::model::EdgeToWallet;

#[utoipa::path(
    post,
    path = "/wallets",
    tag = "wallets",
    responses(
        (status = 200, description = "success response", body = EdgeToWallet)
    )
)]
pub async fn provision_edge_wallet(
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<EdgeToWallet>>, AppError> {
    // let result = state.service.ed.get_token(device_id).await?;
    // Ok(Json(result))
    todo!()
}
