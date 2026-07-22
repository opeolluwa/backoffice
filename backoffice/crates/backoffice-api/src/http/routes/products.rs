use std::sync::Arc;

use axum::Router;
use axum::middleware;
use axum::routing::get;

use crate::http::handlers::products::{create_product, find_all_products, find_product};
use crate::http::middlewares::auth::authenticate;
use crate::state::AppState;

pub(super) fn product_routes(state: Arc<AppState>) -> Router {
    let router = Router::new()
        .route("/", get(find_all_products).post(create_product))
        .route("/{identifier}", get(find_product));

    Router::new()
        .nest("/products", router)
        .layer(middleware::from_fn(authenticate))
        .with_state(state)
}
