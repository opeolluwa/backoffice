use axum::{
    Router, middleware,
    routing::{delete, get, post, put},
};
use std::sync::Arc;

use crate::api::{
    http::{
        handlers::invitation::{
            block_invitation, count_invitations, create_invitation, delete_invitation,
            find_all_invitations, find_invitation_by_identifier,
        },
        middlewares::auth::authenticate,
    },
    state::AppState,
};

pub(super) fn invitation_routes(state: Arc<AppState>) -> Router {
    let routes = Router::new()
        .route("/", post(create_invitation))
        .route("/", get(find_all_invitations))
        .route("/count", get(count_invitations))
        .route("/{identifier}", get(find_invitation_by_identifier))
        .route("/{identifier}/block", put(block_invitation))
        .route("/{identifier}", delete(delete_invitation));

    Router::new()
        .nest("/invitations", routes)
        .layer(middleware::from_fn(authenticate))
        .with_state(state)
}
