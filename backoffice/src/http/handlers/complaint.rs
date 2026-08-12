use std::sync::Arc;

use axum::{
    extract::{Path, State},
    http::StatusCode,
};

use crate::dto::{CreateComplaintCommand, UpdateComplaintCommand};
use crate::errors::api_response::ApiResponse;
use crate::errors::service_error::ServiceError;
use crate::models::complaints;
use crate::services::complaint_services::ComplaintServiceExt;

use crate::http::dto::api_request::AuthenticatedRequest;
use crate::http::dto::jwt::Claims;
use crate::http::extractors::complaint::{CreateComplaintRequest, UpdateComplaintRequest};
use crate::state::AppState;

pub async fn create_complaint(
    State(state): State<Arc<AppState>>,
    request: AuthenticatedRequest<CreateComplaintRequest>,
) -> Result<ApiResponse<complaints::Model>, ServiceError> {
    let command = CreateComplaintCommand {
        customer_identifier: request.data.customer_identifier,
        order_identifier: request.data.order_identifier,
        subject: request.data.subject,
        description: request.data.description,
    };
    let result = state
        .services
        .complaint_service
        .create_complaint(command)
        .await?;

    Ok(ApiResponse::builder()
        .message("complaint created successfully")
        .status_code(StatusCode::CREATED)
        .data(result)
        .build())
}

pub async fn find_all_complaints(
    State(state): State<Arc<AppState>>,
    _claims: Claims,
) -> Result<
    ApiResponse<Vec<(complaints::Model, Option<crate::models::customers::Model>)>>,
    ServiceError,
> {
    let results = state
        .services
        .complaint_service
        .find_all_complaints()
        .await?;

    Ok(ApiResponse::builder()
        .message("complaints fetched successfully")
        .data(results)
        .build())
}

pub async fn find_complaint_by_identifier(
    State(state): State<Arc<AppState>>,
    _claims: Claims,
    Path(identifier): Path<String>,
) -> Result<ApiResponse<(complaints::Model, Option<crate::models::customers::Model>)>, ServiceError>
{
    let result = state
        .services
        .complaint_service
        .find_complaint_by_identifier(&identifier)
        .await?;

    Ok(ApiResponse::builder()
        .message("complaint fetched successfully")
        .data(result)
        .build())
}

pub async fn update_complaint(
    State(state): State<Arc<AppState>>,
    Path(identifier): Path<String>,
    AuthenticatedRequest { data, .. }: AuthenticatedRequest<UpdateComplaintRequest>,
) -> Result<ApiResponse<complaints::Model>, ServiceError> {
    let command = UpdateComplaintCommand {
        subject: data.subject,
        description: data.description,
        status: data.status,
    };
    let result = state
        .services
        .complaint_service
        .update_complaint(&identifier, command)
        .await?;

    Ok(ApiResponse::builder()
        .message("complaint updated successfully")
        .data(result)
        .build())
}

pub async fn count_complaints(
    State(state): State<Arc<AppState>>,
    _claims: Claims,
) -> Result<ApiResponse<i64>, ServiceError> {
    let count = state.services.complaint_service.count_complaints().await?;

    Ok(ApiResponse::builder()
        .message("complaints counted successfully")
        .data(count)
        .build())
}

pub async fn delete_complaint(
    State(state): State<Arc<AppState>>,
    _claims: Claims,
    Path(identifier): Path<String>,
) -> Result<ApiResponse<()>, ServiceError> {
    state
        .services
        .complaint_service
        .delete_complaint(&identifier)
        .await?;

    Ok(ApiResponse::builder()
        .message("complaint deleted successfully")
        .build())
}
