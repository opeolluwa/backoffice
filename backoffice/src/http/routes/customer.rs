use std::sync::Arc;

use axum::middleware;
use axum::routing::delete;
use axum::{Router, routing::get};

use crate::http::handlers::customer::{
    count_customers, delete_customer_by_identifier, find_all_customers, find_customer_by_identifier,
};
use crate::http::middlewares::auth::authenticate;
use crate::state::AppState;

pub(super) fn customer_routes(state: Arc<AppState>) -> Router {
    let routes = Router::new()
        .route("/", get(find_all_customers))
        .route("/{identifier}", get(find_customer_by_identifier))
        .route("/count", get(count_customers))
        .route("/{identifier}", delete(delete_customer_by_identifier));

    Router::new()
        .nest("/customers", routes)
        .layer(middleware::from_fn(authenticate))
        .with_state(state)
}
