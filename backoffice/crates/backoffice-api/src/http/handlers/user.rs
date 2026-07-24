use std::sync::Arc;

use axum::extract::{Multipart, State};
use axum::http::StatusCode;

use backoffice_domain::dto::{ChangePasswordCommand, UpdateProfileCommand};
use backoffice_domain::errors::api_response::{ApiResponse, ApiResponseBuilder};
use backoffice_domain::errors::service_error::ServiceError;
use backoffice_domain::ports::image_uploader::ImageUploader;
use backoffice_domain::services::user::UserServiceTrait;

use crate::http::dto::api_request::AuthenticatedRequest;
use crate::http::dto::jwt::Claims;
use crate::http::extractors::user::{ChangePasswordRequest, UpdateProfileRequest};
use crate::state::AppState;

pub async fn retrieve_information(
    State(state): State<Arc<AppState>>,
    claim: Claims,
) -> Result<ApiResponse<backoffice_domain::dto::UserProfile>, ServiceError> {
    let user_data = state
        .services
        .user_service
        .retrieve_information(&claim.identifier)
        .await?;

    Ok(ApiResponseBuilder::new()
        .data(user_data)
        .message("User's profile fetched successfully")
        .build())
}

pub async fn update_profile(
    State(state): State<Arc<AppState>>,
    AuthenticatedRequest { data, claims }: AuthenticatedRequest<UpdateProfileRequest>,
) -> Result<ApiResponse<backoffice_domain::dto::UserProfile>, ServiceError> {
    let command = UpdateProfileCommand {
        first_name: data.first_name,
        last_name: data.last_name,
        username: data.username,
    };

    state
        .services
        .user_service
        .update_profile(&claims.identifier, &command)
        .await?;

    let user_data = state
        .services
        .user_service
        .retrieve_information(&claims.identifier)
        .await?;

    Ok(ApiResponseBuilder::new()
        .status_code(StatusCode::OK)
        .data(user_data)
        .message("Profile updated successfully")
        .build())
}

pub async fn change_password(
    State(state): State<Arc<AppState>>,
    AuthenticatedRequest { data, claims }: AuthenticatedRequest<ChangePasswordRequest>,
) -> Result<ApiResponse<()>, ServiceError> {
    let command = ChangePasswordCommand {
        current_password: data.current_password,
        new_password: data.new_password,
        confirm_password: data.confirm_password,
    };

    state
        .services
        .user_service
        .change_password(&claims.identifier, &command)
        .await?;

    Ok(ApiResponseBuilder::new()
        .status_code(StatusCode::OK)
        .data(())
        .message("Password changed successfully")
        .build())
}

pub async fn update_profile_picture(
    State(state): State<Arc<AppState>>,
    claim: Claims,
    mut multipart: Multipart,
) -> Result<ApiResponse<backoffice_domain::dto::UserProfile>, ServiceError> {
    let mut file_data: Option<Vec<u8>> = None;
    let mut file_name: Option<String> = None;

    while let Some(field) = multipart
        .next_field()
        .await
        .map_err(|e| ServiceError::OperationFailed(e.to_string()))?
    {
        let field_name = field.name().unwrap_or_default().to_string();
        if field_name == "file" {
            file_name = field.file_name().map(|f| f.to_string());
            file_data = Some(
                field
                    .bytes()
                    .await
                    .map_err(|e| ServiceError::OperationFailed(e.to_string()))?
                    .to_vec(),
            );
        }
    }

    let contents = file_data.ok_or_else(|| {
        ServiceError::OperationFailed("no file provided".to_string())
    })?;
    let name = file_name.unwrap_or_else(|| "profile-picture".to_string());

    let temp_path = std::env::temp_dir().join(&name);
    std::fs::write(&temp_path, &contents)
        .map_err(|e| ServiceError::OperationFailed(e.to_string()))?;

    let upload_result = ImageUploader::upload_file(
        &state.services.imagekit,
        &temp_path,
        &name,
    )
    .await?;

    let _ = std::fs::remove_file(&temp_path);

    state
        .services
        .user_service
        .update_profile_picture(&claim.identifier, &upload_result.url)
        .await?;

    let user_data = state
        .services
        .user_service
        .retrieve_information(&claim.identifier)
        .await?;

    Ok(ApiResponseBuilder::new()
        .status_code(StatusCode::OK)
        .data(user_data)
        .message("Profile picture updated successfully")
        .build())
}
