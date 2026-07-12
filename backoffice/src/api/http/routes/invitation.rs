use std::sync::Arc;

use axum::{
    Router,
    routing::{delete, get, post, put},
};

use crate::api::{
    http::handlers::invitation::{
        accept_invitation, block_invitation, count_invitations, create_invitation,
        delete_invitation, find_all_invitations, find_invitation_by_identifier,
    },
    state::AppState,
};

pub(super) fn invitation_routes(state: Arc<AppState>) -> Router {
    let routes = Router::new()
        .route("/", post(create_invitation))
        .route("/", get(find_all_invitations))
        .route("/count", get(count_invitations))
        .route("/{identifier}", get(find_invitation_by_identifier))
        .route("/{identifier}/accept", put(accept_invitation))
        .route("/{identifier}/block", put(block_invitation))
        .route("/{identifier}", delete(delete_invitation));

    Router::new()
        .nest("/invitations", routes)
        .with_state(state)
}
