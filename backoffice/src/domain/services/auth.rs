use askama::Template;

use crate::infrastructure::mailer::{
    AutoRespondTemplate, EmailRequestBuilder, PasswordResetTemplate, ZeptoMail,
};

use crate::api::http::extractors::dto::jwt::{Claims, JwtCredentials, TEN_MINUTES, TWENTY_FIVE_MINUTES};
use crate::errors::database_error::DatabaseError;
use crate::errors::service_error::ServiceError;
use crate::{
    api::http::extractors::{
        requests::auth::{
            CreateUserRequest, ForgottenPasswordRequest, LoginRequest, RefreshTokenRequest,
            SetNewPasswordRequest, VerifyAccountRequest,
        },
        responses::auth::{
            ForgottenPasswordResponse, LoginResponse, RefreshTokenResponse, SetNewPasswordResponse,
            VerifyAccountResponse,
        },
    },
    domain::{
        ports::user_repository::UserRepositoryTrait,
        services::user_helper::{UserHelperService, UserHelperServiceTrait},
    },
    errors::auth_service_error::AuthenticationServiceError,
};

#[derive(Clone)]
pub struct AuthenticationService<R: UserRepositoryTrait> {
    repo: R,
    user_helper_service: UserHelperService,
    email_client: ZeptoMail,
}

impl<R: UserRepositoryTrait + Clone> AuthenticationService<R> {
    pub fn new(repo: R, email_client: ZeptoMail) -> Self {
        Self {
            repo,
            user_helper_service: UserHelperService::init(),
            email_client,
        }
    }
}

pub trait AuthenticationServiceTrait {
    fn create_user(
        &self,
        request: &CreateUserRequest,
    ) -> impl std::future::Future<Output = Result<(), ServiceError>> + Send;

    fn login(
        &self,
        request: &LoginRequest,
    ) -> impl std::future::Future<Output = Result<LoginResponse, AuthenticationServiceError>> + Send;

    fn forgotten_password(
        &self,
        request: &ForgottenPasswordRequest,
    ) -> impl std::future::Future<
        Output = Result<ForgottenPasswordResponse, AuthenticationServiceError>,
    > + Send;

    fn set_new_password(
        &self,
        request: &SetNewPasswordRequest,
        claims: &Claims,
    ) -> impl std::future::Future<
        Output = Result<SetNewPasswordResponse, AuthenticationServiceError>,
    > + Send;

    fn verify_account(
        &self,
        claims: &Claims,
        request: &VerifyAccountRequest,
    ) -> impl std::future::Future<Output = Result<VerifyAccountResponse, AuthenticationServiceError>>
    + Send;

    fn request_refresh_token(
        &self,
        request: &RefreshTokenRequest,
    ) -> impl std::future::Future<Output = Result<RefreshTokenResponse, AuthenticationServiceError>>
    + Send;
}

impl<R: UserRepositoryTrait + Clone + Send + Sync> AuthenticationServiceTrait
    for AuthenticationService<R>
{
    async fn create_user(&self, request: &CreateUserRequest) -> Result<(), ServiceError> {
        if self.repo.find_by_email(&request.email).await.is_some() {
            return Err(DatabaseError::DuplicateEmailForUser.into());
        }

        let password_hash = self.user_helper_service.hash_password(&request.password)?;
        let user = CreateUserRequest {
            password: password_hash,
            first_name: request.first_name.to_owned(),
            email: request.email.to_owned(),
            last_name: request.last_name.to_owned(),
        };

        self.repo.create_user(user).await.map_err(|err| {
            log::error!("{}", err);
            err
        })?;

        let email_client = self.email_client.clone();
        let user_email = request.email.clone();
        let user_name = request.first_name.clone();
        let email_body = AutoRespondTemplate { name: &user_name }.render().unwrap();

        tokio::task::spawn(async move {
            let email_request = EmailRequestBuilder::new()
                .from("noreply@backoffice.app", "Paula")
                .to(user_email, user_name)
                .subject("Welcome to Backoffice")
                .html_body(email_body)
                .build();

            if let Err(err) = email_client.send_email(email_request).await {
                log::error!("Failed to send welcome email: {}", err);
            }
        });

        Ok(())
    }

    async fn login(
        &self,
        request: &LoginRequest,
    ) -> Result<LoginResponse, AuthenticationServiceError> {
        let Some(user) = self.repo.find_by_email(&request.email).await else {
            return Err(AuthenticationServiceError::WrongCredentials);
        };

        let valid_password = self
            .user_helper_service
            .validate_password(&request.password, &user.password)?;
        if !valid_password {
            return Err(AuthenticationServiceError::WrongCredentials);
        }

        let token =
            JwtCredentials::new(&user.email, &user.identifier).generate_token(TEN_MINUTES)?;

        Ok(LoginResponse { token })
    }

    async fn forgotten_password(
        &self,
        request: &ForgottenPasswordRequest,
    ) -> Result<ForgottenPasswordResponse, AuthenticationServiceError> {
        let Some(user) = self.repo.find_by_email(&request.email).await else {
            return Err(AuthenticationServiceError::WrongCredentials);
        };

        let token =
            JwtCredentials::new(&user.email, &user.identifier).generate_token(TEN_MINUTES)?;

        let email_body = PasswordResetTemplate {
            reset_link: &format!("https://yourapp.com/reset-password?token={token}"),
            name: user.first_name.as_deref().unwrap_or("there"),
        }
        .render()
        .unwrap();

        let email_client = self.email_client.clone();
        let user_email = user.email.clone();
        let user_name = user.first_name.clone().unwrap_or_else(|| "there".into());
        let email_body = email_body.clone();

        tokio::task::spawn(async move {
            let email_request = EmailRequestBuilder::new()
                .from("noreply@backoffice.app", "Paula")
                .to(user_email, user_name)
                .subject("Password Reset")
                .html_body(email_body)
                .build();

            if let Err(err) = email_client.send_email(email_request).await {
                log::error!("Failed to send password reset email: {}", err);
            }
        });

        Ok(ForgottenPasswordResponse { token })
    }

    async fn set_new_password(
        &self,
        request: &SetNewPasswordRequest,
        claims: &Claims,
    ) -> Result<SetNewPasswordResponse, AuthenticationServiceError> {
        let new_password = self.user_helper_service.hash_password(&request.password)?;

        if self.repo.find_by_identifier(&claims.identifier).await.is_none() {
            return Err(AuthenticationServiceError::InvalidToken);
        };

        self.repo
            .update_password(&claims.identifier, &new_password)
            .await?;

        Ok(SetNewPasswordResponse {})
    }

    async fn verify_account(
        &self,
        claims: &Claims,
        _request: &VerifyAccountRequest,
    ) -> Result<VerifyAccountResponse, AuthenticationServiceError> {
        if self.repo.find_by_identifier(&claims.identifier).await.is_none() {
            return Err(AuthenticationServiceError::InvalidToken);
        };

        //todo: validate account credentials
        self.repo.update_account_status(&claims.identifier).await?;
        Ok(VerifyAccountResponse {})
    }

    async fn request_refresh_token(
        &self,
        request: &RefreshTokenRequest,
    ) -> Result<RefreshTokenResponse, AuthenticationServiceError> {
        let refresh_token = JwtCredentials::new(&request.email, &request.identifier)
            .generate_token(TWENTY_FIVE_MINUTES)?;

        Ok(RefreshTokenResponse {
            token: refresh_token,
        })
    }
}
