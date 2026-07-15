use std::sync::Arc;

use axum::extract::State;
use axum::http::StatusCode;

use crate::api::http::dto::api_response::ApiResponseBuilder;
use crate::api::http::dto::jwt::Claims;
use crate::api::http::extractors::auth::VerifyAccountRequest;
use crate::api::http::extractors::auth::{ForgottenPasswordResponse, RefreshTokenResponse};
use crate::api::http::middlewares::validator::ValidatedRequest;
use crate::api::state::AppState;
use crate::{
    api::http::dto::api_response::ApiResponse,
    api::http::extractors::auth::{
        CreateUserRequest, CreateUserResponse, ForgottenPasswordRequest, LoginRequest,
        LoginResponse, SetNewPasswordRequest, VerifyAccountResponse,
    },
    domain::services::auth::AuthenticationServiceTrait,
    errors::auth_service_error::AuthenticationServiceError,
};

pub async fn create_account(
    State(state): State<Arc<AppState>>,
    ValidatedRequest(request): ValidatedRequest<CreateUserRequest>,
) -> Result<ApiResponse<CreateUserResponse>, AuthenticationServiceError> {
    state.services.auth_service.create_user(&request).await?;

    Ok(ApiResponseBuilder::new()
        .status_code(StatusCode::CREATED)
        .message("Account created successfully")
        .build())
}

pub async fn login(
    State(state): State<Arc<AppState>>,
    ValidatedRequest(request): ValidatedRequest<LoginRequest>,
) -> Result<ApiResponse<LoginResponse>, AuthenticationServiceError> {
    let login_response = state.services.auth_service.login(&request).await?;
    Ok(ApiResponseBuilder::new()
        .status_code(StatusCode::OK)
        .data(login_response)
        .message("logged in successfully")
        .build())
}

pub async fn verify_account(
    State(state): State<Arc<AppState>>,
    claims: Claims,
    ValidatedRequest(request): ValidatedRequest<VerifyAccountRequest>,
) -> Result<ApiResponse<VerifyAccountResponse>, AuthenticationServiceError> {
    let verify_account_response = state
        .services
        .auth_service
        .verify_account(&claims, &request)
        .await?;
    Ok(ApiResponseBuilder::new()
        .status_code(StatusCode::OK)
        .data(verify_account_response)
        .build())
}

pub async fn forgotten_password(
    State(state): State<Arc<AppState>>,
    ValidatedRequest(request): ValidatedRequest<ForgottenPasswordRequest>,
) -> Result<ApiResponse<ForgottenPasswordResponse>, AuthenticationServiceError> {
    let forgotten_password_response = state
        .services
        .auth_service
        .forgotten_password(&request)
        .await?;

    Ok(ApiResponseBuilder::new()
        .data(forgotten_password_response)
        .message("account retrival instructions has been sent to the registered email address")
        .build())
}

pub async fn set_new_password(
    State(state): State<Arc<AppState>>,
    claims: Claims,
    ValidatedRequest(request): ValidatedRequest<SetNewPasswordRequest>,
) -> Result<ApiResponse<()>, AuthenticationServiceError> {
    let _ = state
        .services
        .auth_service
        .set_new_password(&request, &claims)
        .await?;

    Ok(ApiResponseBuilder::new()
        .data(())
        .message("password updated successfully")
        .build())
}

pub async fn request_refresh_token(
    State(state): State<Arc<AppState>>,
    claims: Claims,
) -> Result<ApiResponse<RefreshTokenResponse>, AuthenticationServiceError> {
    let refresh_token_response = state
        .services
        .auth_service
        .request_refresh_token(&claims)
        .await?;

    Ok(ApiResponseBuilder::new()
        .data(refresh_token_response)
        .message("token updated successfully")
        .build())
}
