use std::sync::Arc;
use axum::middleware;
use crate::api::http::middlewares::auth::authenticate;
use axum::{
    Router,
    routing::{delete, get, post, put},
};

use crate::api::{
    http::handlers::upload::{
        count_uploads, create_upload, delete_upload, find_all_uploads,
        find_starred_uploads, find_upload_by_identifier, update_upload,
    },
    state::AppState,
};

pub(super) fn upload_routes(state: Arc<AppState>) -> Router {
    let routes = Router::new()
        .route("/", post(create_upload))
        .route("/", get(find_all_uploads))
        .route("/count", get(count_uploads))
        .route("/starred", get(find_starred_uploads))
        .route("/{identifier}", get(find_upload_by_identifier))
        .route("/{identifier}", put(update_upload))
        .route("/{identifier}", delete(delete_upload));

    Router::new()
        .nest("/uploads", routes)
        .layer(middleware::from_fn(authenticate))
        .with_state(state)
}
