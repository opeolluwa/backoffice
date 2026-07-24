use std::sync::Arc;

use axum::middleware;
use axum::{Router, routing::{get, post, put}};

use crate::http::handlers::user::{
    change_password, retrieve_information, update_profile, update_profile_picture,
};
use crate::http::middlewares::auth::authenticate;
use crate::state::AppState;

pub(super) fn user_routes(state: Arc<AppState>) -> Router {
    let routes = Router::new()
        .route("/profile", get(retrieve_information))
        .route("/profile", put(update_profile))
        .route("/profile-picture", post(update_profile_picture))
        .route("/change-password", post(change_password));

    Router::new()
        .nest("/users", routes)
        .layer(middleware::from_fn(authenticate))
        .with_state(state)
}
