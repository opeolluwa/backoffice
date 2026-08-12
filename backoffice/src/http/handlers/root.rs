use std::sync::Arc;

use axum::extract::State;

use crate::errors::api_response::{ApiResponse, ApiResponseBuilder};
use crate::errors::app_error::AppError;
use crate::services::root_services::RootServiceTrait;

use crate::state::AppState;

pub async fn health_check(State(state): State<Arc<AppState>>) -> Result<ApiResponse<()>, AppError> {
    state.services.root_service.health_check()?;
    Ok(ApiResponseBuilder::new()
        .message("service is healthy")
        .build())
}
