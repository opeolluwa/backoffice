#[cfg(feature = "http")]
use axum::{http::StatusCode, response::IntoResponse};
#[cfg(feature = "http")]
use sea_orm::sqlx;

#[cfg(feature = "http")]
use crate::errors::api_response::ApiResponseBuilder;

#[derive(thiserror::Error, Debug)]
pub enum DatabaseError {
    #[error("record not found: {0}")]
    NotFound(String),
    #[error("duplicate record: {0}")]
    DuplicateEntry(String),
    #[error("invalid data: {0}")]
    InvalidData(String),
    #[error("database operation failed: {0}")]
    OperationFailed(String),
    #[cfg(feature = "http")]
    #[error(transparent)]
    SqlxError(#[from] sqlx::Error),
    #[cfg(feature = "http")]
    #[error(transparent)]
    SeaOrmError(#[from] sea_orm::DbErr),
    #[error("A user with the same email already exists")]
    DuplicateEmailForUser,
}

impl DatabaseError {
    #[cfg(feature = "http")]
    pub fn status_code(&self) -> StatusCode {
        match self {
            Self::NotFound(_) => StatusCode::NOT_FOUND,
            Self::DuplicateEntry(_) => StatusCode::CONFLICT,
            Self::InvalidData(_) => StatusCode::BAD_REQUEST,
            Self::OperationFailed(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::SqlxError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::SeaOrmError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::DuplicateEmailForUser => StatusCode::CONFLICT,
        }
    }
}

#[cfg(feature = "http")]
impl IntoResponse for DatabaseError {
    fn into_response(self) -> axum::response::Response {
        ApiResponseBuilder::<()>::new()
            .status_code(self.status_code())
            .message(&self.to_string())
            .build()
            .into_response()
    }
}
