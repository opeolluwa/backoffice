use crate::{
    dto::{
        CreateUserCommand, ForgottenPasswordCommand, ForgottenPasswordResult,
        LoginCommand, LoginResult, SetNewPasswordCommand, SetNewPasswordResult,
        TokenClaims, VerifyAccountCommand, VerifyAccountResult, RefreshTokenCommand,
        RefreshTokenResult, EmailMessage,
    },
    errors::{auth_service_error::AuthenticationServiceError, service_error::ServiceError},
    ports::{
        email_sender::EmailSender, token_service::TokenService,
        user_repository::UserRepositoryTrait,
    },
    services::user_helper::{UserHelperService, UserHelperServiceTrait},
};

pub struct AuthenticationService<R: UserRepositoryTrait, T: TokenService, E: EmailSender> {
    repo: R,
    user_helper_service: UserHelperService,
    token_service: T,
    email_sender: E,
}

impl<R: UserRepositoryTrait, T: TokenService, E: EmailSender> AuthenticationService<R, T, E> {
    pub fn new(repo: R, token_service: T, email_sender: E) -> Self {
        Self {
            repo,
            user_helper_service: UserHelperService::init(),
            token_service,
            email_sender,
        }
    }
}

pub trait AuthenticationServiceTrait {
    fn create_user(
        &self,
        command: &CreateUserCommand,
    ) -> impl std::future::Future<Output = Result<(), ServiceError>> + Send;

    fn login(
        &self,
        command: &LoginCommand,
    ) -> impl std::future::Future<Output = Result<LoginResult, AuthenticationServiceError>> + Send;

    fn forgotten_password(
        &self,
        command: &ForgottenPasswordCommand,
    ) -> impl std::future::Future<
        Output = Result<ForgottenPasswordResult, AuthenticationServiceError>,
    > + Send;

    fn set_new_password(
        &self,
        command: &SetNewPasswordCommand,
        claims: &TokenClaims,
    ) -> impl std::future::Future<
        Output = Result<SetNewPasswordResult, AuthenticationServiceError>,
    > + Send;

    fn verify_account(
        &self,
        claims: &TokenClaims,
        command: &VerifyAccountCommand,
    ) -> impl std::future::Future<Output = Result<VerifyAccountResult, AuthenticationServiceError>>
    + Send;

    fn request_refresh_token(
        &self,
        command: &RefreshTokenCommand,
    ) -> impl std::future::Future<Output = Result<RefreshTokenResult, AuthenticationServiceError>> + Send;
}

impl<R: UserRepositoryTrait + Send + Sync, T: TokenService, E: EmailSender> AuthenticationServiceTrait for AuthenticationService<R, T, E> {
    async fn create_user(&self, command: &CreateUserCommand) -> Result<(), ServiceError> {
        if self.repo.find_by_email(&command.email).await.is_some() {
            return Err(crate::errors::database_error::DatabaseError::DuplicateEmailForUser.into());
        }

        let password_hash = self.user_helper_service.hash_password(&command.password)?;
        let user = CreateUserCommand {
            password: password_hash,
            first_name: command.first_name.to_owned(),
            email: command.email.to_owned(),
            last_name: command.last_name.to_owned(),
        };

        self.repo.create_user(user).await.map_err(|err| {
            tracing::error!("{}", err);
            err
        })?;

        let user_email = command.email.clone();
        let user_name = command.first_name.clone();

        let email_sender = &self.email_sender;
        let message = EmailMessage {
            from_address: "noreply@backoffice.app".to_string(),
            from_name: "Paula".to_string(),
            to_address: user_email,
            to_name: user_name,
            subject: "Welcome to Backoffice".to_string(),
            html_body: format!("Welcome to Backoffice!"),
        };

        if let Err(err) = email_sender.send_email(message).await {
            tracing::error!("Failed to send welcome email: {}", err);
        }

        Ok(())
    }

    async fn login(
        &self,
        command: &LoginCommand,
    ) -> Result<LoginResult, AuthenticationServiceError> {
        let Some(user) = self.repo.find_by_email(&command.email).await else {
            return Err(AuthenticationServiceError::WrongCredentials);
        };

        let valid_password = self
            .user_helper_service
            .validate_password(&command.password, &user.password)?;
        if !valid_password {
            return Err(AuthenticationServiceError::WrongCredentials);
        }

        let claims = TokenClaims {
            email: user.email.clone(),
            identifier: user.identifier.clone(),
        };
        let token = self.token_service.generate_token(&claims, 600)?;

        Ok(LoginResult { token })
    }

    async fn forgotten_password(
        &self,
        command: &ForgottenPasswordCommand,
    ) -> Result<ForgottenPasswordResult, AuthenticationServiceError> {
        let Some(user) = self.repo.find_by_email(&command.email).await else {
            return Err(AuthenticationServiceError::WrongCredentials);
        };

        let claims = TokenClaims {
            email: user.email.clone(),
            identifier: user.identifier.clone(),
        };
        let token = self.token_service.generate_token(&claims, 600)?;

        let reset_link = format!("https://yourapp.com/reset-password?token={token}");
        let user_name = user.first_name.as_deref().unwrap_or("there");

        let message = EmailMessage {
            from_address: "noreply@backoffice.app".to_string(),
            from_name: "Paula".to_string(),
            to_address: user.email.clone(),
            to_name: user.first_name.clone().unwrap_or_else(|| "there".into()),
            subject: "Password Reset".to_string(),
            html_body: format!(
                "Hi {}, click here to reset your password: {}",
                user_name, reset_link
            ),
        };

        if let Err(err) = self.email_sender.send_email(message).await {
            tracing::error!("Failed to send password reset email: {}", err);
        }

        Ok(ForgottenPasswordResult { token })
    }

    async fn set_new_password(
        &self,
        command: &SetNewPasswordCommand,
        claims: &TokenClaims,
    ) -> Result<SetNewPasswordResult, AuthenticationServiceError> {
        let new_password = self.user_helper_service.hash_password(&command.password)?;

        if self
            .repo
            .find_by_identifier(&claims.identifier)
            .await
            .is_none()
        {
            return Err(AuthenticationServiceError::InvalidToken);
        };

        self.repo
            .update_password(&claims.identifier, &new_password)
            .await?;

        Ok(SetNewPasswordResult {})
    }

    async fn verify_account(
        &self,
        claims: &TokenClaims,
        _command: &VerifyAccountCommand,
    ) -> Result<VerifyAccountResult, AuthenticationServiceError> {
        if self
            .repo
            .find_by_identifier(&claims.identifier)
            .await
            .is_none()
        {
            return Err(AuthenticationServiceError::InvalidToken);
        };

        self.repo.update_account_status(&claims.identifier).await?;
        Ok(VerifyAccountResult {})
    }

    async fn request_refresh_token(
        &self,
        command: &RefreshTokenCommand,
    ) -> Result<RefreshTokenResult, AuthenticationServiceError> {
        let claims = TokenClaims {
            email: command.email.clone(),
            identifier: command.identifier.clone(),
        };
        let refresh_token = self.token_service.generate_token(&claims, 1500)?;

        Ok(RefreshTokenResult {
            token: refresh_token,
        })
    }
}
