use axum::extract::State;

use crate::errors::service_error::ServiceError;
use crate::services::user_service::UserServiceTrait;
use crate::{
    adapters::{
        dto::{jwt::Claims, user::UserDto},
        response::api_response::{ApiResponse, ApiResponseBuilder},
    },
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
