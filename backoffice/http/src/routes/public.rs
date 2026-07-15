use std::sync::Arc;

use axum::{
    Router,
    routing::{get, put},
};

use crate::api::http::handlers::{invitation::accept_invitation, root::health_check};
use crate::api::state::AppState;
use crate::api::{
    http::handlers::countries::{fetch_all_countries, fetch_country_by_identifier},

};

pub(super) fn public_routes(state: Arc<AppState>) -> Router {
    
    Router::new()
        .route("/health", get(health_check))
        .route("/invitations/{identifier}/accept", put(accept_invitation))
        .route("/countries", get(fetch_all_countries))
        .route("/countries/{identifier}", get(fetch_country_by_identifier))
        .with_state(state)
}
