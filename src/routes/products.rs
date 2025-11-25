use axum::Router;
use axum::routing::{get, post};

use crate::controllers::products::{
    add_product_to_marketplace, fetch_product_from_marketplace, retrieve_product_from_marketplace,
};

use crate::states::services_state::ServicesState;

pub(super) fn product_routes(state: ServicesState) -> Router {
    Router::new()
        .route("/", post(add_product_to_marketplace))
        .route("/", get(fetch_product_from_marketplace))
        .route("/{identifier}", get(retrieve_product_from_marketplace))
        .with_state(state)
}
