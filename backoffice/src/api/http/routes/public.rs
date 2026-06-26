use axum::{Router, routing::get};

use crate::api::http::handlers::root::health_check;
use crate::states::ServicesState;

pub(super) fn public_routes(state: ServicesState) -> Router {
    Router::new()
        .route("/health", get(health_check))
        .with_state(state)
}
