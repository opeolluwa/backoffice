use std::sync::Arc;

use axum::{
    extract::{Path, State},
    http::StatusCode,
};

use crate::{
    api::http::dto::{api_request::AuthenticatedRequest, api_response::ApiResponse, jwt::Claims},
    api::http::extractors::email::{CreateEmailRequest, UpdateEmailRequest},
    api::state::AppState,
    domain::models::emails,
    domain::services::emails::EmailsServiceExt,
    errors::service_error::ServiceError,
};

pub async fn create_email(
    State(state): State<Arc<AppState>>,
    request: AuthenticatedRequest<CreateEmailRequest>,
) -> Result<ApiResponse<emails::Model>, ServiceError> {
    let email = state
        .services
        .emails_service
        .create_email(&request.data, &request.claims.identifier)
        .await?;
    Ok(ApiResponse::builder()
        .message("Email created successfully")
        .status_code(StatusCode::CREATED)
        .data(email)
        .build())
}

pub async fn find_all_emails(
    State(state): State<Arc<AppState>>,
    claims: Claims,
) -> Result<ApiResponse<Vec<emails::Model>>, ServiceError> {
    let emails = state
        .services
        .emails_service
        .find_all_emails(&claims.identifier)
        .await?;
    Ok(ApiResponse::builder()
        .message("Emails fetched successfully")
        .data(emails)
        .build())
}

pub async fn find_email_by_identifier(
    State(state): State<Arc<AppState>>,
    claims: Claims,
    Path(identifier): Path<String>,
) -> Result<ApiResponse<emails::Model>, ServiceError> {
    let email = state
        .services
        .emails_service
        .find_email_by_identifier(&identifier, &claims.identifier)
        .await?;
    Ok(ApiResponse::builder()
        .message("Email fetched successfully")
        .data(email)
        .build())
}

pub async fn find_emails_by_tag(
    State(state): State<Arc<AppState>>,
    claims: Claims,
    Path(tag): Path<String>,
) -> Result<ApiResponse<Vec<emails::Model>>, ServiceError> {
    let emails = state
        .services
        .emails_service
        .find_emails_by_tag(&tag, &claims.identifier)
        .await?;
    Ok(ApiResponse::builder()
        .message("Emails by tag fetched successfully")
        .data(emails)
        .build())
}

pub async fn find_starred_emails(
    State(state): State<Arc<AppState>>,
    claims: Claims,
) -> Result<ApiResponse<Vec<emails::Model>>, ServiceError> {
    let emails = state
        .services
        .emails_service
        .find_starred_emails(&claims.identifier)
        .await?;
    Ok(ApiResponse::builder()
        .message("Starred emails fetched successfully")
        .data(emails)
        .build())
}

pub async fn find_unread_emails(
    State(state): State<Arc<AppState>>,
    claims: Claims,
) -> Result<ApiResponse<Vec<emails::Model>>, ServiceError> {
    let emails = state
        .services
        .emails_service
        .find_unread_emails(&claims.identifier)
        .await?;
    Ok(ApiResponse::builder()
        .message("Unread emails fetched successfully")
        .data(emails)
        .build())
}

pub async fn update_email(
    State(state): State<Arc<AppState>>,
    Path(identifier): Path<String>,
    AuthenticatedRequest { data, claims }: AuthenticatedRequest<UpdateEmailRequest>,
) -> Result<ApiResponse<emails::Model>, ServiceError> {
    let email = state
        .services
        .emails_service
        .update_email(&identifier, &data, &claims.identifier)
        .await?;
    Ok(ApiResponse::builder()
        .message("Email updated successfully")
        .data(email)
        .build())
}

pub async fn delete_email(
    State(state): State<Arc<AppState>>,
    claims: Claims,
    Path(identifier): Path<String>,
) -> Result<ApiResponse<()>, ServiceError> {
    state
        .services
        .emails_service
        .delete_email(&identifier, &claims.identifier)
        .await?;
    Ok(ApiResponse::builder()
        .message("Email deleted successfully")
        .build())
}

pub async fn count_emails(
    State(state): State<Arc<AppState>>,
    claims: Claims,
) -> Result<ApiResponse<i64>, ServiceError> {
    let count = state
        .services
        .emails_service
        .count_emails(&claims.identifier)
        .await?;
    Ok(ApiResponse::builder()
        .message("Emails counted successfully")
        .data(count)
        .build())
}

pub async fn count_unread_emails(
    State(state): State<Arc<AppState>>,
    claims: Claims,
) -> Result<ApiResponse<i64>, ServiceError> {
    let count = state
        .services
        .emails_service
        .count_unread_emails(&claims.identifier)
        .await?;
    Ok(ApiResponse::builder()
        .message("Unread emails counted successfully")
        .data(count)
        .build())
}
