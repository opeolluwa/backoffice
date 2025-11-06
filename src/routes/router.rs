use axum::{Router, http::StatusCode, response::IntoResponse};
use axum::routing::get_service;
use sqlx::{Pool, Postgres};
use tower_http::services::ServeDir;

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

    Router::new()
        .merge(public_routes(state.clone()))
        .merge(authentication_routes(state.clone()))
        .nest("/users", user_routes(state.clone()))
        .nest_service("/assets", ServeDir::new("assets"))
        // .fallback(async || {
        //     ApiResponseBuilder::<()>::new()
        //         .message("the resource you're looking does not exist or it has been permanently moved")
        //         .status_code(StatusCode::NOT_FOUND)
        //         .build()
        //         .into_response()
        // })
}
