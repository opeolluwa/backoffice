use std::sync::Arc;

use axum::{
    extract::{Path, State},
    http::StatusCode,
};

use crate::{
    api::http::dto::{api_request::AuthenticatedRequest, api_response::ApiResponse, jwt::Claims},
    api::http::extractors::team::{CreateTeamMemberRequest, UpdateTeamMemberRequest},
    api::state::AppState,
    domain::services::team::TeamServiceExt,
    entities::teams,
    errors::service_error::ServiceError,
};

pub async fn create_team_member(
    State(state): State<Arc<AppState>>,
    request: AuthenticatedRequest<CreateTeamMemberRequest>,
) -> Result<ApiResponse<teams::Model>, ServiceError> {
    let member = state
        .services
        .team_service
        .create_team_member(&request.data)
        .await?;
    Ok(ApiResponse::builder()
        .message("Team member created successfully")
        .status_code(StatusCode::CREATED)
        .data(member)
        .build())
}

pub async fn find_all_team_members(
    State(state): State<Arc<AppState>>,
    _claims: Claims,
) -> Result<ApiResponse<Vec<teams::Model>>, ServiceError> {
    let members = state.services.team_service.find_all_team_members().await?;
    Ok(ApiResponse::builder()
        .message("Team members fetched successfully")
        .data(members)
        .build())
}

pub async fn find_team_member_by_identifier(
    State(state): State<Arc<AppState>>,
    _claims: Claims,
    Path(identifier): Path<String>,
) -> Result<ApiResponse<teams::Model>, ServiceError> {
    let member = state
        .services
        .team_service
        .find_team_member_by_identifier(&identifier)
        .await?;
    Ok(ApiResponse::builder()
        .message("Team member fetched successfully")
        .data(member)
        .build())
}

pub async fn update_team_member(
    State(state): State<Arc<AppState>>,
    Path(identifier): Path<String>,
    AuthenticatedRequest { data, claims: _ }: AuthenticatedRequest<UpdateTeamMemberRequest>,
) -> Result<ApiResponse<teams::Model>, ServiceError> {
    let member = state
        .services
        .team_service
        .update_team_member(&identifier, &data)
        .await?;
    Ok(ApiResponse::builder()
        .message("Team member updated successfully")
        .data(member)
        .build())
}

pub async fn delete_team_member(
    State(state): State<Arc<AppState>>,
    _claims: Claims,
    Path(identifier): Path<String>,
) -> Result<ApiResponse<()>, ServiceError> {
    state
        .services
        .team_service
        .delete_team_member(&identifier)
        .await?;
    Ok(ApiResponse::builder()
        .message("Team member deleted successfully")
        .build())
}

pub async fn block_team_member(
    State(state): State<Arc<AppState>>,
    _claims: Claims,
    Path(identifier): Path<String>,
) -> Result<ApiResponse<teams::Model>, ServiceError> {
    let member = state
        .services
        .team_service
        .block_team_member(&identifier, true)
        .await?;
    Ok(ApiResponse::builder()
        .message("Team member blocked successfully")
        .data(member)
        .build())
}

pub async fn unblock_team_member(
    State(state): State<Arc<AppState>>,
    _claims: Claims,
    Path(identifier): Path<String>,
) -> Result<ApiResponse<teams::Model>, ServiceError> {
    let member = state
        .services
        .team_service
        .block_team_member(&identifier, false)
        .await?;
    Ok(ApiResponse::builder()
        .message("Team member unblocked successfully")
        .data(member)
        .build())
}

pub async fn count_team_members(
    State(state): State<Arc<AppState>>,
    _claims: Claims,
) -> Result<ApiResponse<i64>, ServiceError> {
    let count = state.services.team_service.count_team_members().await?;
    Ok(ApiResponse::builder()
        .message("Team members counted successfully")
        .data(count)
        .build())
}
