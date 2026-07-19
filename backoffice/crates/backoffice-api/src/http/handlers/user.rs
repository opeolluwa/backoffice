use std::sync::Arc;

use axum::extract::State;

use backoffice_domain::errors::api_response::{ApiResponse, ApiResponseBuilder};
use backoffice_domain::errors::service_error::ServiceError;
use backoffice_domain::services::user::UserServiceTrait;

use crate::http::dto::jwt::Claims;
use crate::state::AppState;

pub async fn retrieve_information(
    State(state): State<Arc<AppState>>,
    claim: Claims,
) -> Result<ApiResponse<backoffice_domain::dto::UserProfile>, ServiceError> {
    let user_data = state
        .services
        .user_service
        .retrieve_information(&claim.identifier)
        .await?;

    Ok(ApiResponseBuilder::new()
        .data(user_data)
        .message("User's profile fetched successfully")
        .build())
}
