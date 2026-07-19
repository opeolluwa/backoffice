#[cfg(feature = "http")]
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

#[cfg(feature = "http")]
use crate::errors::api_response::ApiResponseBuilder;

#[derive(Debug, thiserror::Error)]
pub enum AppFileSystemError {
    #[error("Failed to save file on disk")]
    FailedToSaveToDisk,

    #[error("Failed to upload file")]
    UploadFailed,
}

impl AppFileSystemError {
    #[cfg(feature = "http")]
    pub fn status_code(&self) -> StatusCode {
        StatusCode::UNPROCESSABLE_ENTITY
    }
}

#[cfg(feature = "http")]
impl IntoResponse for AppFileSystemError {
    fn into_response(self) -> Response {
        ApiResponseBuilder::<()>::new()
            .status_code(self.status_code())
            .message(&self.to_string())
            .build()
            .into_response()
    }
}
