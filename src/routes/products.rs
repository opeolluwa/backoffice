use axum::Router;

use crate::states::services_state::ServicesState;

pub(super) fn product_routes(state: ServicesState) -> Router {
    Router::new().with_state(state)
}
