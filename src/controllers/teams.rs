use axum::{
    extract::{Path, State},
    http::StatusCode,
};

use crate::{
    adapters::{
        dto::jwt::Claims,
        requests::{
            api_request::AuthenticatedRequest,
            team::{CreateTeamMemberRequest, UpdateTeamMemberRequest},
        },
        response::api_response::ApiResponse,
    },
    entities::teams,
    errors::service_error::ServiceError,
    services::team_service::{TeamService, TeamServiceExt},
};

pub async fn create_team_member(
    State(team_service): State<TeamService>,
    request: AuthenticatedRequest<CreateTeamMemberRequest>,
) -> Result<ApiResponse<teams::Model>, ServiceError> {
    let member = team_service.create_team_member(&request.data).await?;
    Ok(ApiResponse::builder()
        .message("Team member created successfully")
        .status_code(StatusCode::CREATED)
        .data(member)
        .build())
}

pub async fn find_all_team_members(
    State(team_service): State<TeamService>,
    _claims: Claims,
) -> Result<ApiResponse<Vec<teams::Model>>, ServiceError> {
    let members = team_service.find_all_team_members().await?;
    Ok(ApiResponse::builder()
        .message("Team members fetched successfully")
        .data(members)
        .build())
}

pub async fn find_team_member_by_identifier(
    State(team_service): State<TeamService>,
    _claims: Claims,
    Path(identifier): Path<String>,
) -> Result<ApiResponse<teams::Model>, ServiceError> {
    let member = team_service
        .find_team_member_by_identifier(&identifier)
        .await?;
    Ok(ApiResponse::builder()
        .message("Team member fetched successfully")
        .data(member)
        .build())
}

pub async fn update_team_member(
    State(team_service): State<TeamService>,
    Path(identifier): Path<String>,
    AuthenticatedRequest { data, claims: _ }: AuthenticatedRequest<UpdateTeamMemberRequest>,
) -> Result<ApiResponse<teams::Model>, ServiceError> {
    let member = team_service.update_team_member(&identifier, &data).await?;
    Ok(ApiResponse::builder()
        .message("Team member updated successfully")
        .data(member)
        .build())
}

pub async fn delete_team_member(
    State(team_service): State<TeamService>,
    _claims: Claims,
    Path(identifier): Path<String>,
) -> Result<ApiResponse<()>, ServiceError> {
    team_service.delete_team_member(&identifier).await?;
    Ok(ApiResponse::builder()
        .message("Team member deleted successfully")
        .build())
}

pub async fn block_team_member(
    State(team_service): State<TeamService>,
    _claims: Claims,
    Path(identifier): Path<String>,
) -> Result<ApiResponse<teams::Model>, ServiceError> {
    let member = team_service.block_team_member(&identifier, true).await?;
    Ok(ApiResponse::builder()
        .message("Team member blocked successfully")
        .data(member)
        .build())
}

pub async fn unblock_team_member(
    State(team_service): State<TeamService>,
    _claims: Claims,
    Path(identifier): Path<String>,
) -> Result<ApiResponse<teams::Model>, ServiceError> {
    let member = team_service.block_team_member(&identifier, false).await?;
    Ok(ApiResponse::builder()
        .message("Team member unblocked successfully")
        .data(member)
        .build())
}

pub async fn count_team_members(
    State(team_service): State<TeamService>,
    _claims: Claims,
) -> Result<ApiResponse<i64>, ServiceError> {
    let count = team_service.count_team_members().await?;
    Ok(ApiResponse::builder()
        .message("Team members counted successfully")
        .data(count)
        .build())
}
