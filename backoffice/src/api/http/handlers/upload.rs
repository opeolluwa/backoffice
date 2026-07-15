use std::sync::Arc;

use axum::{
    extract::{Path, State},
    http::StatusCode,
};
use axum_typed_multipart::TypedMultipart;

use crate::{
    api::http::dto::{api_request::AuthenticatedRequest, api_response::ApiResponse, jwt::Claims},
    api::http::extractors::upload::{CreateUploadRequest, UpdateUploadRequest},
    api::state::AppState,
    domain::models::uploads,
    domain::services::upload::UploadsServiceExt,
    errors::service_error::ServiceError,
};

pub async fn create_upload(
    State(state): State<Arc<AppState>>,
    _claims: Claims,
    request: TypedMultipart<CreateUploadRequest>,
) -> Result<ApiResponse<uploads::Model>, ServiceError> {
    let upload = state
        .services
        .upload_service
        .create_upload(request)
        .await?;
    Ok(ApiResponse::builder()
        .message("Upload created successfully")
        .status_code(StatusCode::CREATED)
        .data(upload)
        .build())
}

pub async fn find_all_uploads(
    State(state): State<Arc<AppState>>,
) -> Result<ApiResponse<Vec<uploads::Model>>, ServiceError> {
    let uploads = state.services.upload_service.find_all_uploads().await?;
    Ok(ApiResponse::builder()
        .message("Uploads fetched successfully")
        .data(uploads)
        .build())
}

pub async fn find_starred_uploads(
    State(state): State<Arc<AppState>>,
) -> Result<ApiResponse<Vec<uploads::Model>>, ServiceError> {
    let uploads = state
        .services
        .upload_service
        .find_starred_uploads()
        .await?;
    Ok(ApiResponse::builder()
        .message("Starred uploads fetched successfully")
        .data(uploads)
        .build())
}

pub async fn find_upload_by_identifier(
    State(state): State<Arc<AppState>>,
    Path(identifier): Path<String>,
) -> Result<ApiResponse<uploads::Model>, ServiceError> {
    let upload = state
        .services
        .upload_service
        .find_upload_by_identifier(&identifier)
        .await?;
    Ok(ApiResponse::builder()
        .message("Upload fetched successfully")
        .data(upload)
        .build())
}

pub async fn update_upload(
    State(state): State<Arc<AppState>>,
    Path(identifier): Path<String>,
    AuthenticatedRequest { data, .. }: AuthenticatedRequest<UpdateUploadRequest>,
) -> Result<ApiResponse<uploads::Model>, ServiceError> {
    let upload = state
        .services
        .upload_service
        .update_upload(&identifier, &data)
        .await?;
    Ok(ApiResponse::builder()
        .message("Upload updated successfully")
        .data(upload)
        .build())
}

pub async fn delete_upload(
    State(state): State<Arc<AppState>>,
    Path(identifier): Path<String>,
) -> Result<ApiResponse<()>, ServiceError> {
    state
        .services
        .upload_service
        .delete_upload(&identifier)
        .await?;
    Ok(ApiResponse::builder()
        .message("Upload deleted successfully")
        .build())
}

pub async fn count_uploads(
    State(state): State<Arc<AppState>>,
) -> Result<ApiResponse<i64>, ServiceError> {
    let count = state.services.upload_service.count_uploads().await?;
    Ok(ApiResponse::builder()
        .message("Uploads counted successfully")
        .data(count)
        .build())
}
