pub mod dto;
mod handler;
pub mod model;
mod service;

pub use handler::*;
pub use service::*;

use std::sync::Arc;

use axum::routing::get;

use crate::state::AppState;

pub fn routes<S>(state: Arc<AppState>) -> axum::Router<S> {
    axum::Router::new()
        .route("/tokens/:pubkey", get(handler::get_token_asset))
        .with_state(state)
}
