use std::sync::Arc;

use axum::{
    extract::{Path, State},
    http::StatusCode,
};
use axum_typed_multipart::TypedMultipart;
use tracing::{debug, info};

use crate::dto::UpdateUploadCommand;
use crate::errors::api_response::ApiResponse;
use crate::errors::service_error::ServiceError;
use crate::models::uploads;
use crate::services::upload_services::UploadsServiceExt;

use crate::http::dto::api_request::AuthenticatedRequest;
use crate::http::extractors::upload::{CreateUploadRequest, UpdateUploadRequest};
use crate::state::AppState;

fn to_update_command(req: &UpdateUploadRequest) -> UpdateUploadCommand {
    UpdateUploadCommand {
        name: req.name.clone(),
        starred: req.starred,
    }
}

pub async fn create_upload(
    State(state): State<Arc<AppState>>,
    TypedMultipart(CreateUploadRequest {
        file,
        name,
        file_type: _,
        starred,
    }): TypedMultipart<CreateUploadRequest>,
) -> Result<ApiResponse<uploads::Model>, ServiceError> {
    let file_name = if name.is_empty() {
        chrono::Local::now().format("%Y%m%d%H%M%S").to_string()
    } else {
        name.clone()
    };

    let file_path = file.contents.path().to_path_buf();
    let file_size = std::fs::metadata(&file_path).map(|m| m.len()).unwrap_or(0);

    debug!(
        file_name = %file_name,
        file_size,
        name = %name,
        starred = starred.unwrap_or(false),
        "create_upload: processing multipart upload",
    );

    let upload = state
        .services
        .upload_service
        .create_upload(file_path, &file_name, &name, starred.unwrap_or(false))
        .await?;

    info!(
        upload_id = %upload.identifier,
        "create_upload: upload persisted successfully",
    );

    Ok(ApiResponse::builder()
        .message("Upload created successfully")
        .status_code(StatusCode::CREATED)
        .data(upload)
        .build())
}

pub async fn find_all_uploads(
    State(state): State<Arc<AppState>>,
) -> Result<ApiResponse<Vec<uploads::Model>>, ServiceError> {
    debug!("find_all_uploads: fetching all uploads");
    let uploads = state.services.upload_service.find_all_uploads().await?;
    debug!(count = uploads.len(), "find_all_uploads: returned");
    Ok(ApiResponse::builder()
        .message("Uploads fetched successfully")
        .data(uploads)
        .build())
}

pub async fn find_starred_uploads(
    State(state): State<Arc<AppState>>,
) -> Result<ApiResponse<Vec<uploads::Model>>, ServiceError> {
    debug!("find_starred_uploads: fetching starred uploads");
    let uploads = state.services.upload_service.find_starred_uploads().await?;
    debug!(count = uploads.len(), "find_starred_uploads: returned");
    Ok(ApiResponse::builder()
        .message("Starred uploads fetched successfully")
        .data(uploads)
        .build())
}

pub async fn find_upload_by_identifier(
    State(state): State<Arc<AppState>>,
    Path(identifier): Path<String>,
) -> Result<ApiResponse<uploads::Model>, ServiceError> {
    debug!(identifier = %identifier, "find_upload_by_identifier: looking up upload");
    let upload = state
        .services
        .upload_service
        .find_upload_by_identifier(&identifier)
        .await?;
    debug!(identifier = %identifier, "find_upload_by_identifier: found");
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
    debug!(
        identifier = %identifier,
        name = ?data.name,
        starred = data.starred,
        "update_upload: applying changes",
    );
    let command = to_update_command(&data);
    let upload = state
        .services
        .upload_service
        .update_upload(&identifier, &command)
        .await?;
    info!(identifier = %identifier, "update_upload: done");
    Ok(ApiResponse::builder()
        .message("Upload updated successfully")
        .data(upload)
        .build())
}

pub async fn delete_upload(
    State(state): State<Arc<AppState>>,
    Path(identifier): Path<String>,
) -> Result<ApiResponse<()>, ServiceError> {
    debug!(identifier = %identifier, "delete_upload: removing upload");
    state
        .services
        .upload_service
        .delete_upload(&identifier)
        .await?;
    info!(identifier = %identifier, "delete_upload: done");
    Ok(ApiResponse::builder()
        .message("Upload deleted successfully")
        .build())
}

pub async fn count_uploads(
    State(state): State<Arc<AppState>>,
) -> Result<ApiResponse<i64>, ServiceError> {
    debug!("count_uploads: counting");
    let count = state.services.upload_service.count_uploads().await?;
    debug!(count, "count_uploads: done");
    Ok(ApiResponse::builder()
        .message("Uploads counted successfully")
        .data(count)
        .build())
}
