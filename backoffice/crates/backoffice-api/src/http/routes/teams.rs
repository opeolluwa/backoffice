use std::sync::Arc;

use axum::middleware;
use axum::{
    Router,
    routing::{delete, get, post, put},
};

use crate::http::handlers::teams::{
    block_team_member, count_team_members, create_team_member, delete_team_member,
    find_all_team_members, find_team_member_by_identifier, unblock_team_member, update_team_member,
};
use crate::http::middlewares::auth::authenticate;
use crate::state::AppState;

pub(super) fn team_routes(state: Arc<AppState>) -> Router {
    let routes = Router::new()
        .route("/", post(create_team_member))
        .route("/", get(find_all_team_members))
        .route("/count", get(count_team_members))
        .route("/{identifier}", get(find_team_member_by_identifier))
        .route("/{identifier}", put(update_team_member))
        .route("/{identifier}", delete(delete_team_member))
        .route("/{identifier}/block", put(block_team_member))
        .route("/{identifier}/unblock", put(unblock_team_member));

    Router::new()
        .nest("/teams", routes)
        .layer(middleware::from_fn(authenticate))
        .with_state(state)
}
