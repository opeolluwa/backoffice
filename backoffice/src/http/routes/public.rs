use std::sync::Arc;

use axum::{Router, routing::get};

use crate::http::handlers::root::health_check;
use crate::state::AppState;

pub(super) fn public_routes(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/health", get(health_check))
        .with_state(state)
}
