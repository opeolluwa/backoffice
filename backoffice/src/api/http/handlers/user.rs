use axum::extract::State;

use crate::api::http::extractors::user::UserDto;
use crate::errors::service_error::ServiceError;
use crate::services::user_service::UserServiceTrait;
use crate::{
    api::http::dto::api_response::{ApiResponse, ApiResponseBuilder},
    api::http::dto::jwt::Claims,
    services::user_service::UserService,
};

pub async fn retrieve_information(
    State(user_service): State<UserService>,
    claim: Claims,
) -> Result<ApiResponse<UserDto>, ServiceError> {
    let user_data = user_service.retrieve_information(&claim.identifier).await?;

    Ok(ApiResponseBuilder::new()
        .data(user_data)
        .message("User's profile fetched successfully")
        .build())
}
