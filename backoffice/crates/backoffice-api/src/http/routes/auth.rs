use std::sync::Arc;
use std::time::Duration;

use axum::{
    Router,
    middleware,
    routing::post,
};

use crate::http::handlers::auth::{
    create_account, forgotten_password, login, logout, request_refresh_token, set_new_password,
    verify_account,
};
use crate::http::middlewares::auth::authenticate;
use crate::http::middlewares::rate_limit::RateLimitLayer;
use crate::state::AppState;

pub(super) fn authentication_routes(state: Arc<AppState>) -> Router {
    let auth_rate_limit = RateLimitLayer::new(20, Duration::from_secs(60));

    let public_routes = Router::new()
        .route("/signup", post(create_account))
        .route("/login", post(login))
        .route("/forgotten-password", post(forgotten_password))
        .route("/refresh-token", post(request_refresh_token))
        .layer(auth_rate_limit);

    let protected_routes = Router::new()
        .route("/reset-password", post(set_new_password))
        .route("/verify-account", post(verify_account))
        .route("/logout", post(logout))
        .layer(middleware::from_fn(authenticate));

    Router::new()
        .merge(public_routes)
        .merge(protected_routes)
        .with_state(state)
}
