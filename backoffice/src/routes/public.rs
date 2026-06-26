use axum::{Router, routing::get};

use crate::{controllers::root::health_check, states::ServicesState};

pub(super) fn public_routes(state: ServicesState) -> Router {
    Router::new()
        .route("/health", get(health_check))
        .with_state(state)
}
