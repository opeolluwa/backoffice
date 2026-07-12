use axum::{http::StatusCode, response::IntoResponse, response::Response};

use crate::api::http::dto::api_response::ApiResponseBuilder;

#[derive(thiserror::Error, Debug)]
pub enum EmailServiceError {
    #[error("failed to send email")]
    SendEmailError,
    #[error("failed due to {0}")]
    OperationFailed(String),
}

impl EmailServiceError {
    pub fn status_code(&self) -> StatusCode {
        match self {
            Self::SendEmailError => StatusCode::INTERNAL_SERVER_ERROR,
            Self::OperationFailed(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl IntoResponse for EmailServiceError {
    fn into_response(self) -> Response {
        ApiResponseBuilder::<()>::new()
            .status_code(self.status_code())
            .message(&self.to_string())
            .build()
            .into_response()
    }
}
