use std::sync::Arc;

use axum::extract::State;

use crate::{
    api::http::dto::api_response::{ApiResponse, ApiResponseBuilder},
    api::http::dto::jwt::Claims,
    api::http::extractors::user::UserDto,
    api::state::AppState,
    domain::services::user::UserServiceTrait,
    errors::service_error::ServiceError,
};

pub async fn retrieve_information(
    State(state): State<Arc<AppState>>,
    claim: Claims,
) -> Result<ApiResponse<UserDto>, ServiceError> {
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
