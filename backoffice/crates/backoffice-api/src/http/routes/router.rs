use std::sync::Arc;

use axum::{Router, extract::Extension};
use tower_http::services::{ServeDir, ServeFile};

use crate::http::routes::{
    auth::authentication_routes, complaint::complaint_routes, config::config_routes,
    country::country_routes, customer::customer_routes, email::email_routes,
    invitation::invitation_routes, orders::orders_routes, products::product_routes,
    public::public_routes, teams::team_routes, uploads::upload_routes, users::user_routes,
};
use crate::state::AppState;

pub fn load_routes(app_state: AppState) -> Router {
    let serve_dir = ServeDir::new("assets").not_found_service(ServeFile::new("assets/index.html"));
    let state = Arc::new(app_state);

    Router::new()
        .nest(
            "/api",
            Router::new()
                .merge(user_routes(Arc::clone(&state)))
                .merge(country_routes(Arc::clone(&state)))
                .merge(public_routes(Arc::clone(&state)))
                .merge(authentication_routes(Arc::clone(&state)))
                .merge(product_routes(Arc::clone(&state)))
                .merge(team_routes(Arc::clone(&state)))
                .merge(email_routes(Arc::clone(&state)))
                .merge(invitation_routes(Arc::clone(&state)))
                .merge(upload_routes(Arc::clone(&state)))
                .merge(orders_routes(Arc::clone(&state)))
                .merge(customer_routes(Arc::clone(&state)))
                .merge(complaint_routes(Arc::clone(&state)))
                .merge(config_routes(Arc::clone(&state))),
        )
        .layer(Extension(state.redis.clone()))
        .fallback_service(serve_dir)
}
