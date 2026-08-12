use std::sync::Arc;

use axum::middleware;
use axum::routing::{delete, post, put};
use axum::{Router, routing::get};

use crate::http::handlers::complaint::{
    count_complaints, create_complaint, delete_complaint, find_all_complaints,
    find_complaint_by_identifier, update_complaint,
};
use crate::http::middlewares::auth::authenticate;
use crate::state::AppState;

pub(super) fn complaint_routes(state: Arc<AppState>) -> Router {
    let routes = Router::new()
        .route("/", post(create_complaint))
        .route("/", get(find_all_complaints))
        .route("/{identifier}", get(find_complaint_by_identifier))
        .route("/{identifier}", put(update_complaint))
        .route("/count", get(count_complaints))
        .route("/{identifier}", delete(delete_complaint));

    Router::new()
        .nest("/complaints", routes)
        .layer(middleware::from_fn(authenticate))
        .with_state(state)
}
