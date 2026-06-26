use axum::{
    Router,
    routing::{delete, get, post, put},
};

use crate::{
    controllers::teams::{
        block_team_member, count_team_members, create_team_member, delete_team_member,
        find_all_team_members, find_team_member_by_identifier, unblock_team_member,
        update_team_member,
    },
    states::ServicesState,
};

pub(super) fn team_routes(state: ServicesState) -> Router {
    let routes = Router::new()
        .route("/", post(create_team_member))
        .route("/", get(find_all_team_members))
        .route("/count", get(count_team_members))
        .route("/{identifier}", get(find_team_member_by_identifier))
        .route("/{identifier}", put(update_team_member))
        .route("/{identifier}", delete(delete_team_member))
        .route("/{identifier}/block", put(block_team_member))
        .route("/{identifier}/unblock", put(unblock_team_member));

    Router::new().nest("/teams", routes).with_state(state)
}
