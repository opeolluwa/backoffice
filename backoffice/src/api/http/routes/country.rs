use std::sync::Arc;

use axum::{Router, routing::get};

use crate::api::{
    http::handlers::countries::{fetch_all_countries, fetch_country_by_identifier},
    state::AppState,
};

pub(super) fn country_routes(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/countries", get(fetch_all_countries))
        .route("/countries/{identifier}", get(fetch_country_by_identifier))
        .with_state(state)
}
