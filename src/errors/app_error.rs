use axum::{http::StatusCode, response::IntoResponse};

use crate::adapters::response::api_response::ApiResponseBuilder;
use crate::adapters::response::api_response::EmptyResponseBody;

#[derive(thiserror::Error, Debug)]
pub enum AppError {
    #[error("{0}")]
    OperationFailed(String),
    #[error("App failed to start up due to {0}")]
    StartupError(String),
    #[error("Error parsing env due to {0}")]
    EnvError(String),
    #[error(transparent)]
    EnvExtractError(#[from] backoffice_utils::errors::Error),
    #[error(transparent)]
    AppFileSystemError(#[from] std::io::Error),
}

impl AppError {
    pub fn status_code(&self) -> StatusCode {
        StatusCode::INTERNAL_SERVER_ERROR
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        ApiResponseBuilder::<EmptyResponseBody>::new()
            .status_code(self.status_code())
            .message(&self.to_string())
            .build()
            .into_response()
    }
}
