use std::sync::Arc;

use axum::middleware;
use axum::routing::{delete, post, put};
use axum::{Router, routing::get};

use crate::http::handlers::orders::{
    count_orders, create_orders, delete_orders_by_identifier,
    find_all_orders, find_orders_by_identifier, update_orders_by_identifier,
};
use crate::http::middlewares::auth::authenticate;
use crate::state::AppState;

pub(super) fn orders_routes(state: Arc<AppState>) -> Router {
    let routes = Router::new()
        .route("/", post(create_orders))
        .route("/", get(find_all_orders))
        .route("/{identifier}", get(find_orders_by_identifier))
        .route("/count", get(count_orders))
        .route("/{identifier}", put(update_orders_by_identifier))
        .route("/{identifier}", delete(delete_orders_by_identifier));

    Router::new()
        .nest("/orders", routes)
        .layer(middleware::from_fn(authenticate))
        .with_state(state)
}
