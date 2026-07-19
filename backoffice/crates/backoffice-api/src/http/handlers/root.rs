use std::sync::Arc;

use axum::extract::State;

use backoffice_domain::errors::api_response::{ApiResponse, ApiResponseBuilder};
use backoffice_domain::errors::app_error::AppError;
use backoffice_domain::services::root::RootServiceTrait;

use crate::state::AppState;

pub async fn health_check(State(state): State<Arc<AppState>>) -> Result<ApiResponse<()>, AppError> {
    state.services.root_service.health_check()?;
    Ok(ApiResponseBuilder::new()
        .message("service is healthy")
        .build())
}
