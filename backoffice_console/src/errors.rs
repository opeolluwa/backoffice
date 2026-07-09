use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;

#[derive(Debug, thiserror::Error)]
pub enum StartupError {
    #[error("Failed to start server: {0}")]
    ServerFailed(String),

    #[error("Failed to bind to address: {0}")]
    BindFailed(String),

    #[error("Database connection failed: {0}")]
    DatabaseConnectionFailed(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}

impl IntoResponse for StartupError {
    fn into_response(self) -> Response {
        let (status, message) = match &self {
            StartupError::ServerFailed(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg.clone()),
            StartupError::BindFailed(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg.clone()),
            StartupError::DatabaseConnectionFailed(msg) => {
                (StatusCode::SERVICE_UNAVAILABLE, msg.clone())
            }
            StartupError::Io(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("IO error: {}", err),
            ),
        };

        #[derive(Serialize)]
        struct ErrorResponse {
            message: String,
        }

        (status, axum::Json(ErrorResponse { message })).into_response()
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ConfigError {
    #[error("Missing environment variable: {0}")]
    MissingEnv(String),

    #[error("Invalid environment variable {key}: {reason}")]
    InvalidEnv { key: String, reason: String },

    #[error("Failed to load configuration: {0}")]
    LoadFailed(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}

impl IntoResponse for ConfigError {
    fn into_response(self) -> Response {
        let (status, message) = match &self {
            ConfigError::MissingEnv(key) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Missing environment variable: {}", key),
            ),
            ConfigError::InvalidEnv { key, reason } => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Invalid environment variable {}: {}", key, reason),
            ),
            ConfigError::LoadFailed(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg.clone()),
            ConfigError::Io(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("IO error: {}", err),
            ),
        };

        #[derive(Serialize)]
        struct ErrorResponse {
            message: String,
        }

        (status, axum::Json(ErrorResponse { message })).into_response()
    }
}

#[derive(Debug, thiserror::Error)]
pub enum CommandError {
    #[error("Command execution failed: {0}")]
    ExecutionFailed(String),

    #[error("Command not found: {0}")]
    NotFound(String),

    #[error("Invalid arguments: {0}")]
    InvalidArguments(String),

    #[error("Permission denied: {0}")]
    PermissionDenied(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}

impl IntoResponse for CommandError {
    fn into_response(self) -> Response {
        let (status, message) = match &self {
            CommandError::ExecutionFailed(msg) => (StatusCode::BAD_REQUEST, msg.clone()),
            CommandError::NotFound(cmd) => (
                StatusCode::NOT_FOUND,
                format!("Command not found: {}", cmd),
            ),
            CommandError::InvalidArguments(msg) => (StatusCode::BAD_REQUEST, msg.clone()),
            CommandError::PermissionDenied(msg) => (StatusCode::FORBIDDEN, msg.clone()),
            CommandError::Io(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("IO error: {}", err),
            ),
        };

        #[derive(Serialize)]
        struct ErrorResponse {
            message: String,
        }

        (status, axum::Json(ErrorResponse { message })).into_response()
    }
}
