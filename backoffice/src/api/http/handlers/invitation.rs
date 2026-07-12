use std::sync::Arc;

use axum::{
    extract::{Path, State},
    http::StatusCode,
};

use crate::{
    api::http::dto::{api_request::AuthenticatedRequest, api_response::ApiResponse, jwt::Claims},
    api::http::extractors::invitation::{AcceptInvitationRequest, CreateInvitationRequest},
    api::state::AppState,
    domain::models::invitation,
    domain::services::invitation::InvitationServiceExt,
    errors::service_error::ServiceError,
};

pub async fn create_invitation(
    State(state): State<Arc<AppState>>,
    request: AuthenticatedRequest<CreateInvitationRequest>,
) -> Result<ApiResponse<invitation::Model>, ServiceError> {
    let invitation = state
        .services
        .invitation_service
        .create_invitation(&request.data.email)
        .await?;
    Ok(ApiResponse::builder()
        .message("Invitation created successfully")
        .status_code(StatusCode::CREATED)
        .data(invitation)
        .build())
}

pub async fn find_all_invitations(
    State(state): State<Arc<AppState>>,
    _claims: Claims,
) -> Result<ApiResponse<Vec<invitation::Model>>, ServiceError> {
    let invitations = state
        .services
        .invitation_service
        .find_all_invitations()
        .await?;
    Ok(ApiResponse::builder()
        .message("Invitations fetched successfully")
        .data(invitations)
        .build())
}

pub async fn find_invitation_by_identifier(
    State(state): State<Arc<AppState>>,
    _claims: Claims,
    Path(identifier): Path<String>,
) -> Result<ApiResponse<invitation::Model>, ServiceError> {
    let invitation = state
        .services
        .invitation_service
        .find_invitation_by_identifier(&identifier)
        .await?;
    Ok(ApiResponse::builder()
        .message("Invitation fetched successfully")
        .data(invitation)
        .build())
}

pub async fn accept_invitation(
    State(state): State<Arc<AppState>>,
    Path(identifier): Path<String>,
    request: AuthenticatedRequest<AcceptInvitationRequest>,
) -> Result<ApiResponse<invitation::Model>, ServiceError> {
    let invitation = state
        .services
        .invitation_service
        .accept_invitation(&identifier, &request.data.token)
        .await?;
    Ok(ApiResponse::builder()
        .message("Invitation accepted successfully")
        .data(invitation)
        .build())
}

pub async fn block_invitation(
    State(state): State<Arc<AppState>>,
    _claims: Claims,
    Path(identifier): Path<String>,
) -> Result<ApiResponse<invitation::Model>, ServiceError> {
    let invitation = state
        .services
        .invitation_service
        .block_invitation(&identifier)
        .await?;
    Ok(ApiResponse::builder()
        .message("Invitation blocked successfully")
        .data(invitation)
        .build())
}

pub async fn delete_invitation(
    State(state): State<Arc<AppState>>,
    _claims: Claims,
    Path(identifier): Path<String>,
) -> Result<ApiResponse<()>, ServiceError> {
    state
        .services
        .invitation_service
        .delete_invitation(&identifier)
        .await?;
    Ok(ApiResponse::builder()
        .message("Invitation deleted successfully")
        .build())
}

pub async fn count_invitations(
    State(state): State<Arc<AppState>>,
    _claims: Claims,
) -> Result<ApiResponse<i64>, ServiceError> {
    let count = state
        .services
        .invitation_service
        .count_invitations()
        .await?;
    Ok(ApiResponse::builder()
        .message("Invitations counted successfully")
        .data(count)
        .build())
}
