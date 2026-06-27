use std::sync::Arc;

use axum::extract::State;

use crate::api::http::dto::api_response::{ApiResponse, ApiResponseBuilder};
use crate::api::state::AppState;
use crate::domain::services::root::RootServiceTrait;
use crate::errors::app_error::AppError;

pub async fn health_check(
    State(state): State<Arc<AppState>>,
) -> Result<ApiResponse<()>, AppError> {
    state.services.root_service.health_check()?;
    Ok(ApiResponseBuilder::new()
        .message("service is healthy")
        .build())
}
