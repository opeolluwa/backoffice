use axum::{Router, routing::get};

use crate::api::http::handlers::user::retrieve_information;
use crate::states::ServicesState;

pub(super) fn user_routes(state: ServicesState) -> Router {
    Router::new()
        .route("/profile", get(retrieve_information))
        .with_state(state)
}
