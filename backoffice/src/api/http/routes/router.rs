use std::sync::Arc;

use axum::Router;
use tower_http::services::{ServeDir, ServeFile};

use crate::api::{
    http::routes::{
        auth::authentication_routes, country::country_routes, marketplace::marketplace_routes,
        products::product_routes, public::public_routes, teams::team_routes, users::user_routes,
    },
    state::AppState,
};

pub fn load_routes(app_state: AppState) -> Router {
    let serve_dir = ServeDir::new("assets").not_found_service(ServeFile::new("assets/index.html"));
    let state = Arc::new(app_state);

    Router::new()
        .nest(
            "/api",
            Router::new()
                .merge(user_routes(Arc::clone(&state)))
                .merge(public_routes(Arc::clone(&state)))
                .merge(authentication_routes(Arc::clone(&state)))
                .merge(marketplace_routes(Arc::clone(&state)))
                .merge(product_routes(Arc::clone(&state)))
                .merge(country_routes(Arc::clone(&state)))
                .merge(team_routes(Arc::clone(&state))),
        )
        .fallback_service(serve_dir)
}
