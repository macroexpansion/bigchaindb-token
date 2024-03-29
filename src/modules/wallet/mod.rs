pub mod dto;
mod handler;
pub mod model;
mod service;

pub use handler::*;
pub use service::*;

use std::sync::Arc;

use axum::routing::{get, post};

use crate::state::AppState;

pub fn routes<S>(state: Arc<AppState>) -> axum::Router<S> {
    axum::Router::new()
        .route("/wallets/:edge-id", get(handler::get_wallet_by_id))
        .route("/wallets", post(handler::provision_edge_wallet))
        .route("/wallets/transfer", post(handler::transfer_token))
        .with_state(state)
}
