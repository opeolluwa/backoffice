use crate::http::middlewares::auth::authenticate;
use crate::http::handlers::user::retrieve_information;
use crate::state::AppState;
use axum::middleware;
use axum::{Router, routing::get};
use std::sync::Arc;

pub(super) fn user_routes(state: Arc<AppState>) -> Router {
    let routes = Router::new().route("/profile", get(retrieve_information));

    Router::new()
        .nest("/users", routes)
        .layer(middleware::from_fn(authenticate))
        .with_state(state)
}
