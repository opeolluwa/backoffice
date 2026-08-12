use std::sync::Arc;

use axum::{
    extract::{Path, State},
    http::StatusCode,
};

use crate::dto::{CreateEmailCommand, UpdateEmailCommand};
use crate::errors::api_response::ApiResponse;
use crate::errors::service_error::ServiceError;
use crate::models::emails;
use crate::services::emails_services::EmailsServiceExt;

use crate::http::dto::api_request::AuthenticatedRequest;
use crate::http::extractors::email::{CreateEmailRequest, UpdateEmailRequest};
use crate::state::AppState;

fn to_create_command(req: &CreateEmailRequest) -> CreateEmailCommand {
    CreateEmailCommand {
        subject: req.subject.clone(),
        body: req.body.clone(),
        sender_email: req.sender_email.clone(),
        recipient_email: req.recipient_email.clone(),
        tag: req.tag.clone(),
        has_attachments: req.has_attachments,
        data: req.data.clone(),
    }
}

fn to_update_command(req: &UpdateEmailRequest) -> UpdateEmailCommand {
    UpdateEmailCommand {
        tag: req.tag.clone(),
        is_read: req.is_read,
        is_starred: req.is_starred,
    }
}

pub async fn create_email(
    State(state): State<Arc<AppState>>,
    request: AuthenticatedRequest<CreateEmailRequest>,
) -> Result<ApiResponse<emails::Model>, ServiceError> {
    let command = to_create_command(&request.data);
    let email = state.services.emails_service.create_email(&command).await?;
    Ok(ApiResponse::builder()
        .message("Email created successfully")
        .status_code(StatusCode::CREATED)
        .data(email)
        .build())
}

pub async fn find_all_emails(
    State(state): State<Arc<AppState>>,
) -> Result<ApiResponse<Vec<emails::Model>>, ServiceError> {
    let emails = state.services.emails_service.find_all_emails().await?;
    Ok(ApiResponse::builder()
        .message("Emails fetched successfully")
        .data(emails)
        .build())
}

pub async fn find_email_by_identifier(
    State(state): State<Arc<AppState>>,
    Path(identifier): Path<String>,
) -> Result<ApiResponse<emails::Model>, ServiceError> {
    let email = state
        .services
        .emails_service
        .find_email_by_identifier(&identifier)
        .await?;
    Ok(ApiResponse::builder()
        .message("Email fetched successfully")
        .data(email)
        .build())
}

pub async fn find_emails_by_tag(
    State(state): State<Arc<AppState>>,
    Path(tag): Path<String>,
) -> Result<ApiResponse<Vec<emails::Model>>, ServiceError> {
    let emails = state
        .services
        .emails_service
        .find_emails_by_tag(&tag)
        .await?;
    Ok(ApiResponse::builder()
        .message("Emails by tag fetched successfully")
        .data(emails)
        .build())
}

pub async fn find_starred_emails(
    State(state): State<Arc<AppState>>,
) -> Result<ApiResponse<Vec<emails::Model>>, ServiceError> {
    let emails = state.services.emails_service.find_starred_emails().await?;
    Ok(ApiResponse::builder()
        .message("Starred emails fetched successfully")
        .data(emails)
        .build())
}

pub async fn find_unread_emails(
    State(state): State<Arc<AppState>>,
) -> Result<ApiResponse<Vec<emails::Model>>, ServiceError> {
    let emails = state.services.emails_service.find_unread_emails().await?;
    Ok(ApiResponse::builder()
        .message("Unread emails fetched successfully")
        .data(emails)
        .build())
}

pub async fn update_email(
    State(state): State<Arc<AppState>>,
    Path(identifier): Path<String>,
    AuthenticatedRequest { data, .. }: AuthenticatedRequest<UpdateEmailRequest>,
) -> Result<ApiResponse<emails::Model>, ServiceError> {
    let command = to_update_command(&data);
    let email = state
        .services
        .emails_service
        .update_email(&identifier, &command)
        .await?;
    Ok(ApiResponse::builder()
        .message("Email updated successfully")
        .data(email)
        .build())
}

pub async fn delete_email(
    State(state): State<Arc<AppState>>,
    Path(identifier): Path<String>,
) -> Result<ApiResponse<()>, ServiceError> {
    state
        .services
        .emails_service
        .delete_email(&identifier)
        .await?;
    Ok(ApiResponse::builder()
        .message("Email deleted successfully")
        .build())
}

pub async fn count_emails(
    State(state): State<Arc<AppState>>,
) -> Result<ApiResponse<i64>, ServiceError> {
    let count = state.services.emails_service.count_emails().await?;
    Ok(ApiResponse::builder()
        .message("Emails counted successfully")
        .data(count)
        .build())
}

pub async fn count_unread_emails(
    State(state): State<Arc<AppState>>,
) -> Result<ApiResponse<i64>, ServiceError> {
    let count = state.services.emails_service.count_unread_emails().await?;
    Ok(ApiResponse::builder()
        .message("Unread emails counted successfully")
        .data(count)
        .build())
}
