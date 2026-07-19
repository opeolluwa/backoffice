use std::sync::Arc;

use axum::extract::State;
use axum::http::StatusCode;

use backoffice_domain::errors::api_response::ApiResponseBuilder;
use crate::http::dto::jwt::Claims;
use crate::http::middlewares::validator::ValidatedRequest;
use crate::state::AppState;
use backoffice_domain::dto::{
    CreateUserCommand, ForgottenPasswordCommand, LoginCommand, SetNewPasswordCommand,
    TokenClaims, VerifyAccountCommand, RefreshTokenCommand,
};
use backoffice_domain::errors::api_response::ApiResponse;
use crate::http::extractors::auth::{
    CreateUserRequest, ForgottenPasswordRequest, LoginRequest,
    SetNewPasswordRequest, VerifyAccountRequest,
};
use backoffice_domain::services::auth::AuthenticationServiceTrait;
use backoffice_domain::errors::auth_service_error::AuthenticationServiceError;

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
) -> Result<ApiResponse<backoffice_domain::dto::LoginResult>, AuthenticationServiceError> {
    let command = LoginCommand {
        email: request.email,
        password: request.password,
    };
    let login_response = state.services.auth_service.login(&command).await?;
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
) -> Result<ApiResponse<backoffice_domain::dto::VerifyAccountResult>, AuthenticationServiceError> {
    let token_claims = TokenClaims {
        email: claims.email,
        identifier: claims.identifier,
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
) -> Result<ApiResponse<backoffice_domain::dto::ForgottenPasswordResult>, AuthenticationServiceError> {
    let command = ForgottenPasswordCommand { email: request.email };
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
    claims: Claims,
) -> Result<ApiResponse<backoffice_domain::dto::RefreshTokenResult>, AuthenticationServiceError> {
    let command = RefreshTokenCommand {
        email: claims.email,
        identifier: claims.identifier,
    };
    let refresh_token_response = state
        .services
        .auth_service
        .request_refresh_token(&command)
        .await?;

    Ok(ApiResponseBuilder::new()
        .data(refresh_token_response)
        .message("token updated successfully")
        .build())
}
