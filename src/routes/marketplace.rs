use crate::controllers::marketplace::{
    count_marketplaces, create_marketplace, find_marketplace_by_identifier,
};
use crate::states::services_state::ServicesState;
use axum::routing::post;
use axum::{Router, routing::get};

pub(super) fn marketplace_routes(state: ServicesState) -> Router {
    let routes = Router::new()
        .route("/{identifier}", get(find_marketplace_by_identifier))
        .route("/", post(create_marketplace))
        .route("/count", get(count_marketplaces));

    Router::new().nest("/marketplace", routes).with_state(state)
}
