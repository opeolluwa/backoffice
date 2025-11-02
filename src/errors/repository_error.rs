use axum::{http::StatusCode, response::IntoResponse};
use crate::adapters::response::api_response::ApiResponseBuilder;
use crate::errors::common_service_error::ServiceError;

#[derive(thiserror::Error, Debug)]
pub enum RepositoryError {
    #[error("record not found: {0}")]
    NotFound(String),
    #[error("duplicate record: {0}")]
    Duplicate(String),
    #[error("invalid data: {0}")]
    InvalidData(String),
    #[error("database operation failed: {0}")]
    OperationFailed(String),
    #[error(transparent)]
    SqlxError(#[from] sqlx::Error),
    #[error(transparent)]
    ServiceError(#[from] ServiceError),
}

impl RepositoryError {
    pub fn status_code(&self) -> StatusCode {
        match self {
            Self::NotFound(_) => StatusCode::NOT_FOUND,
            Self::Duplicate(_) => StatusCode::CONFLICT,
            Self::InvalidData(_) => StatusCode::BAD_REQUEST,
            Self::OperationFailed(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::SqlxError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::ServiceError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl IntoResponse for RepositoryError {
    fn into_response(self) -> axum::response::Response {
        ApiResponseBuilder::<()>::new()
            .status_code(self.status_code())
            .message(&self.to_string())
            .build()
            .into_response()
    }
}
