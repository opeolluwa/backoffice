use crate::controllers::marketplace::{
    count_marketplaces, create_marketplace, delete_marketplace_by_identifier,
    find_all_marketplaces, find_marketplace_by_identifier, update_marketplace_by_identifier,
};
use crate::states::services_state::ServicesState;
use axum::routing::{delete, post, put};
use axum::{Router, routing::get};

pub(super) fn marketplace_routes(state: ServicesState) -> Router {
    let routes = Router::new()
        .route("/", post(create_marketplace))
        .route("/", get(find_all_marketplaces))
        .route("/{identifier}", get(find_marketplace_by_identifier))
        .route("/count", get(count_marketplaces))
        .route("/{identifier}", put(update_marketplace_by_identifier))
        .route("/{identifier}", delete(delete_marketplace_by_identifier));

    Router::new().nest("/marketplaces", routes).with_state(state)
}
