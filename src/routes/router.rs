use axum::{Router, http::StatusCode, response::IntoResponse};
use axum::routing::get_service;
use sqlx::{Pool, Postgres};
use tower_http::services::{ServeDir, ServeFile};

use crate::{
    adapters::response::api_response::ApiResponseBuilder,
    routes::{auth::authentication_routes, public::public_routes, users::user_routes},
    services::{
        auth_service::AuthenticationService, root_service::RootService, user_service::UserService,
    },
    states::services_state::ServicesState,
};

pub fn load_routes(pool: Pool<Postgres>) -> Router {
    let state = ServicesState {
        user_service: UserService::init(&pool),
        root_service: RootService::init(),
        auth_service: AuthenticationService::init(&pool),
    };

    let serve_dir = ServeDir::new("assets").not_found_service(ServeFile::new("assets/index.html"));

    Router::new()
        .merge(public_routes(state.clone()))
        .merge(authentication_routes(state.clone()))
        .nest("/users", user_routes(state.clone()))
        .fallback_service(serve_dir)
}
