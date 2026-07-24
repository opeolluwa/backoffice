use std::sync::Arc;

use axum::{Router, routing::{get, put}};

use crate::http::handlers::app_config::{fetch_app_config, update_app_config};
use crate::state::AppState;

pub(super) fn config_routes(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/config", get(fetch_app_config))
        .route("/config", put(update_app_config))
        .with_state(state)
}
