use std::sync::Arc;

use axum::{
    extract::{Path, State},
    Json,
};

use crate::{error::AppError, state::AppState};

use super::dto::{EdgeWallet, ProvisionEdgeWallet};

#[utoipa::path(
    get,
    path = "/wallets/{device-id}",
    tag = "wallets",
    params(
        ("device-id" = i32, Path, description = "Device ID"),
    ),
    responses(
        (status = 200, description = "success response", body = EdgeWallet)
    )
)]
pub async fn get_wallet_by_id(
    State(state): State<Arc<AppState>>,
    Path(device_id): Path<i32>,
) -> Result<Json<EdgeWallet>, AppError> {
    let result = state.service.wallet.get_edge_wallet(device_id).await?;
    Ok(Json(result))
}

#[utoipa::path(
    post,
    path = "/wallets",
    tag = "wallets",
    responses(
        (status = 200, description = "success response", body = EdgeWallet)
    )
)]
pub async fn provision_edge_wallet(
    State(state): State<Arc<AppState>>,
    Json(body): Json<ProvisionEdgeWallet>,
) -> Result<Json<EdgeWallet>, AppError> {
    let ProvisionEdgeWallet { edge_id } = body;
    let edge_id = edge_id.parse::<i32>()?;
    let result = state.service.wallet.provision_edge_wallet(edge_id).await?;
    Ok(Json(result))
}
