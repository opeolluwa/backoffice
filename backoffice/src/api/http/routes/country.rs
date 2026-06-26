use axum::{Router, routing::get};

use crate::{
    controllers::countries::{fetch_all_countries, fetch_country_by_identifier},
    states::ServicesState,
};

pub(super) fn country_routes(state: ServicesState) -> Router {
    Router::new()
        .route("/countries", get(fetch_all_countries))
        .route("/countries/{identifier}", get(fetch_country_by_identifier))
        .with_state(state)
}
