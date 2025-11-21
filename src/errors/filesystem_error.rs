use axum::{http::StatusCode, response::{IntoResponse, Response}};

use crate::adapters::response::api_response::ApiResponseBuilder;

#[derive(Debug, thiserror::Error)]
pub enum FileSystemError {
    #[error("Failed to save file on disk")]
    FailedToSaveToDisk,

    #[error("Failed to upload file")]
    UploadFailed,
}

impl FileSystemError {
    pub fn status_code(&self) -> StatusCode {
        StatusCode::UNPROCESSABLE_ENTITY
    }
}


impl IntoResponse for FileSystemError {
    fn into_response(self) -> Response {
        ApiResponseBuilder::<()>::new()
            .status_code(self.status_code())
            .message(&self.to_string())
            .build()
            .into_response()
    }
}
