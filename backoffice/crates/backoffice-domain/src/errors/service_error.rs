#[cfg(feature = "http")]
use axum::extract::rejection::{FormRejection, JsonRejection};
#[cfg(feature = "http")]
use axum::response::Response;
#[cfg(feature = "http")]
use axum::{http::StatusCode, response::IntoResponse};

#[cfg(feature = "http")]
use crate::errors::api_response::ApiResponseBuilder;
use crate::errors::app_error::AppError;
use crate::errors::authentication_error::AuthenticationError;
use crate::errors::database_error::DatabaseError;
use crate::errors::filesystem_error::AppFileSystemError;
use crate::errors::imagekit_error::ImagekitError;

#[derive(thiserror::Error, Debug)]
pub enum ServiceError {
    #[error(transparent)]
    ValidationError(#[from] validator::ValidationErrors),
    #[cfg(feature = "http")]
    #[error(transparent)]
    AxumFormRejection(#[from] FormRejection),
    #[cfg(feature = "http")]
    #[error(transparent)]
    AxumJsonRejection(#[from] JsonRejection),
    #[error("an unknown service error has occurred")]
    Unknown,
    #[error("operation failed: {0}")]
    OperationFailed(String),
    #[error(transparent)]
    AuthenticationError(#[from] AuthenticationError),
    #[error("badly formatted request")]
    BadRequest,
    #[error("an internal error occurred")]
    AppError(#[from] AppError),
    #[error(transparent)]
    DatabaseError(#[from] DatabaseError),
    #[error(transparent)]
    AppFileSystemError(#[from] AppFileSystemError),
    #[error(transparent)]
    FileSystemError(#[from] std::io::Error),
    #[error(transparent)]
    ImagekitError(#[from] ImagekitError),
    #[error(transparent)]
    BcryptError(#[from] bcrypt::BcryptError),
}

impl ServiceError {
    #[cfg(feature = "http")]
    pub fn status_code(&self) -> StatusCode {
        match self {
            ServiceError::ValidationError(_) => StatusCode::BAD_REQUEST,
            ServiceError::AxumFormRejection(_) => StatusCode::BAD_REQUEST,
            ServiceError::AxumJsonRejection(_) => StatusCode::BAD_REQUEST,
            ServiceError::AuthenticationError(error) => error.status_code(),
            ServiceError::DatabaseError(_) => StatusCode::UNPROCESSABLE_ENTITY,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

#[cfg(feature = "http")]
impl IntoResponse for ServiceError {
    fn into_response(self) -> Response {
        tracing::error!(error = %self, "request failed");
        ApiResponseBuilder::<()>::new()
            .status_code(self.status_code())
            .message(&self.to_string())
            .build()
            .into_response()
    }
}
