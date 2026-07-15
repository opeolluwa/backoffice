use axum::{
    Router, middleware,
    routing::{delete, get, post, put},
};
use std::sync::Arc;

use crate::api::{
    http::{
        handlers::invitation::{
            accept_invitation, block_invitation, count_invitations, create_invitation,
            delete_invitation, find_all_invitations, find_invitation_by_identifier,
        },
        middlewares::auth::authenticate,
    },
    state::AppState,
};

pub(super) fn invitation_routes(state: Arc<AppState>) -> Router {
    let unauthenticated_invitation_routes =
        Router::new().route("/{identifier}/accept", put(accept_invitation));

    let routes = Router::new()
        .route("/", post(create_invitation))
        .route("/", get(find_all_invitations))
        .route("/count", get(count_invitations))
        .route("/{identifier}", get(find_invitation_by_identifier))
        .route("/{identifier}/block", put(block_invitation))
        .route("/{identifier}", delete(delete_invitation))
        .layer(middleware::from_fn(authenticate));

    Router::new()
        .nest("/invitations", routes)
        .nest("/invitations", unauthenticated_invitation_routes)
        .with_state(state)
}
