use std::sync::Arc;

use axum::{
    extract::{Path, State},
    Json,
};

use crate::{error::AppError, state::AppState};

use super::dto::{EdgeWallet, ProvisionEdgeWallet, TransferToken};

#[utoipa::path(
    get,
    path = "/wallets/{edge-id}",
    tag = "wallets",
    params(
        ("edge-id" = i32, Path, description = "Edge ID"),
    ),
    responses(
        (status = 200, description = "success response", body = EdgeWallet)
    )
)]
pub async fn get_wallet_by_id(
    State(state): State<Arc<AppState>>,
    Path(edge_id): Path<i32>,
) -> Result<Json<EdgeWallet>, AppError> {
    let result = state.service.wallet.get_edge_wallet(edge_id).await?;
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
    let ProvisionEdgeWallet { edge_id, asset } = body;
    let edge_id = edge_id.parse::<i32>()?;
    let result = state
        .service
        .wallet
        .provision_edge_wallet(edge_id, asset)
        .await?;
    Ok(Json(result))
}

#[utoipa::path(
    post,
    path = "/wallets/transfer",
    tag = "wallets",
    responses(
        (status = 200, description = "success response", body = EdgeWallet)
    )
)]
pub async fn transfer_token(
    State(state): State<Arc<AppState>>,
    Json(body): Json<TransferToken>,
) -> Result<Json<EdgeWallet>, AppError> {
    let TransferToken { edge_id } = body;
    let edge_id = edge_id.parse::<i32>()?;
    let result = state.service.wallet.transfer_token(edge_id).await?;
    Ok(Json(result))
}
