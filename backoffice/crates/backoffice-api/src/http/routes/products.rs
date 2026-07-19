use std::sync::Arc;

use axum::Router;
use axum::middleware;
use axum::routing::{get, post};

use crate::http::handlers::products::{
    add_product_to_marketplace, retrieve_product_from_marketplace,
};
use crate::http::middlewares::auth::authenticate;
use crate::state::AppState;

pub(super) fn product_routes(state: Arc<AppState>) -> Router {
    let router = Router::new()
        .route("/", post(add_product_to_marketplace))
        .route("/{identifier}", get(retrieve_product_from_marketplace));

    Router::new()
        .nest("/products", router)
        .layer(middleware::from_fn(authenticate))
        .with_state(state)
}
