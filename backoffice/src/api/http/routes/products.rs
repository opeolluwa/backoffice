use std::sync::Arc;

use axum::Router;
use axum::routing::{get, post};

use crate::api::http::handlers::products::{
    add_product_to_marketplace, retrieve_product_from_marketplace,
};
use crate::api::state::AppState;

pub(super) fn product_routes(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/", post(add_product_to_marketplace))
        // .route("/", get(fetch_product_from_marketplace))
        .route("/{identifier}", get(retrieve_product_from_marketplace))
        .with_state(state)
}
