use std::sync::Arc;

use axum::extract::{Request, State};
use axum::http::StatusCode;

use crate::dto::{
    CreateUserCommand, ForgottenPasswordCommand, LoginCommand, RefreshTokenCommand,
    SetNewPasswordCommand, TokenClaims, VerifyAccountCommand,
};
use crate::errors::api_response::ApiResponse;
use crate::errors::api_response::ApiResponseBuilder;
use crate::errors::auth_service_error::AuthenticationServiceError;
use crate::services::auth_services::AuthenticationServiceTrait;

use crate::http::dto::jwt::{Claims, REFRESH_TOKEN_DURATION};
use crate::http::extractors::auth::{
    CreateUserRequest, ForgottenPasswordRequest, LoginRequest, RefreshTokenRequest,
    SetNewPasswordRequest, VerifyAccountRequest,
};
use crate::http::middlewares::auth::RawToken;
use crate::http::middlewares::validator::ValidatedRequest;
use crate::state::AppState;

pub async fn create_account(
    State(state): State<Arc<AppState>>,
    ValidatedRequest(request): ValidatedRequest<CreateUserRequest>,
) -> Result<ApiResponse<()>, AuthenticationServiceError> {
    let command = CreateUserCommand {
        email: request.email,
        password: request.password,
        first_name: request.first_name,
        last_name: request.last_name,
    };
    state.services.auth_service.create_user(&command).await?;

    Ok(ApiResponseBuilder::new()
        .status_code(StatusCode::CREATED)
        .message("Account created successfully")
        .build())
}

pub async fn login(
    State(state): State<Arc<AppState>>,
    ValidatedRequest(request): ValidatedRequest<LoginRequest>,
) -> Result<ApiResponse<crate::dto::LoginResult>, AuthenticationServiceError> {
    let command = LoginCommand {
        email: request.email,
        password: request.password,
    };
    let access_token_ttl = state.app_config.access_token_ttl_secs.as_secs();
    let refresh_token_ttl = state.app_config.refresh_token_ttl_secs.as_secs();

    let login_response = state
        .services
        .auth_service
        .login(&command, access_token_ttl, refresh_token_ttl)
        .await?;

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
) -> Result<ApiResponse<crate::dto::VerifyAccountResult>, AuthenticationServiceError> {
    let token_claims = TokenClaims {
        email: claims.email,
        identifier: claims.identifier,
        token_type: claims.token_type,
    };
    let command = VerifyAccountCommand { otp: request.otp };
    let verify_account_response = state
        .services
        .auth_service
        .verify_account(&token_claims, &command)
        .await?;
    Ok(ApiResponseBuilder::new()
        .status_code(StatusCode::OK)
        .data(verify_account_response)
        .build())
}

pub async fn forgotten_password(
    State(state): State<Arc<AppState>>,
    ValidatedRequest(request): ValidatedRequest<ForgottenPasswordRequest>,
) -> Result<ApiResponse<crate::dto::ForgottenPasswordResult>, AuthenticationServiceError> {
    let command = ForgottenPasswordCommand {
        email: request.email,
    };
    let forgotten_password_response = state
        .services
        .auth_service
        .forgotten_password(&command)
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
    let token_claims = TokenClaims {
        email: claims.email,
        identifier: claims.identifier,
        token_type: claims.token_type,
    };
    let command = SetNewPasswordCommand {
        password: request.password,
        confirm_password: request.confirm_password,
    };
    let _ = state
        .services
        .auth_service
        .set_new_password(&command, &token_claims)
        .await?;

    Ok(ApiResponseBuilder::new()
        .data(())
        .message("password updated successfully")
        .build())
}

pub async fn request_refresh_token(
    State(state): State<Arc<AppState>>,
    ValidatedRequest(request): ValidatedRequest<RefreshTokenRequest>,
) -> Result<ApiResponse<crate::dto::RefreshTokenResult>, AuthenticationServiceError> {
    let access_token_ttl = state.app_config.access_token_ttl_secs.as_secs();
    let refresh_token_ttl = state.app_config.refresh_token_ttl_secs.as_secs();

    let command = RefreshTokenCommand {
        refresh_token: request.refresh_token.clone(),
    };

    let refresh_token_response = state
        .services
        .auth_service
        .request_refresh_token(&command, access_token_ttl, refresh_token_ttl)
        .await?;

    // Blacklist the old refresh token (rotation)
    state
        .redis
        .blacklist_token(&request.refresh_token, REFRESH_TOKEN_DURATION.as_secs())
        .map_err(|e| AuthenticationServiceError::OperationFailed(e.to_string()))?;

    Ok(ApiResponseBuilder::new()
        .data(refresh_token_response)
        .message("token refreshed successfully")
        .build())
}

pub async fn logout(
    State(state): State<Arc<AppState>>,
    request: Request,
) -> Result<ApiResponse<()>, AuthenticationServiceError> {
    let raw_token = request
        .extensions()
        .get::<RawToken>()
        .cloned()
        .ok_or(AuthenticationServiceError::MissingCredentials)?;

    state
        .redis
        .blacklist_token(&raw_token.0, REFRESH_TOKEN_DURATION.as_secs())
        .map_err(|e| AuthenticationServiceError::OperationFailed(e.to_string()))?;

    Ok(ApiResponseBuilder::new()
        .status_code(StatusCode::OK)
        .data(())
        .message("logged out successfully")
        .build())
}
