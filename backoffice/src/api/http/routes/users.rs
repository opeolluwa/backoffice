use std::sync::Arc;

use axum::{Router, routing::get};

use crate::api::{http::handlers::user::retrieve_information, state::AppState};

pub(super) fn user_routes(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/profile", get(retrieve_information))
        .with_state(state)
}
