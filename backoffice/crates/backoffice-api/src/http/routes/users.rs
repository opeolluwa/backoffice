use std::sync::Arc;

use axum::middleware;
use axum::{Router, routing::get};

use crate::http::handlers::user::retrieve_information;
use crate::http::middlewares::auth::authenticate;
use crate::state::AppState;

pub(super) fn user_routes(state: Arc<AppState>) -> Router {
    let routes = Router::new().route("/profile", get(retrieve_information));

    Router::new()
        .nest("/users", routes)
        .layer(middleware::from_fn(authenticate))
        .with_state(state)
}
