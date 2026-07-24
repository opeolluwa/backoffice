use crate::{
    dto::{
        CreateUserCommand, ForgottenPasswordCommand, ForgottenPasswordResult, LoginCommand,
        LoginResult, RefreshTokenCommand, RefreshTokenResult, SetNewPasswordCommand,
        SetNewPasswordResult, TokenClaims, VerifyAccountCommand, VerifyAccountResult,
    },
    errors::{auth_service_error::AuthenticationServiceError, service_error::ServiceError},
    ports::{
        email_sender::{EmailMessage, EmailSender},
        token_service::TokenService,
        user_repository::UserRepositoryTrait,
    },
    services::user_helper::{UserHelperService, UserHelperServiceTrait},
};

#[derive(Clone)]
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
        access_token_ttl_secs: u64,
        refresh_token_ttl_secs: u64,
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
    ) -> impl std::future::Future<Output = Result<SetNewPasswordResult, AuthenticationServiceError>> + Send;

    fn verify_account(
        &self,
        claims: &TokenClaims,
        command: &VerifyAccountCommand,
    ) -> impl std::future::Future<Output = Result<VerifyAccountResult, AuthenticationServiceError>> + Send;

    fn request_refresh_token(
        &self,
        command: &RefreshTokenCommand,
        access_token_ttl_secs: u64,
        refresh_token_ttl_secs: u64,
    ) -> impl std::future::Future<Output = Result<RefreshTokenResult, AuthenticationServiceError>> + Send;
}

impl<R: UserRepositoryTrait + Send + Sync, T: TokenService, E: EmailSender + Send + Sync>
    AuthenticationServiceTrait for AuthenticationService<R, T, E>
{
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

        if let Err(err) = email_sender.send_email(message) {
            tracing::error!("Failed to send welcome email: {}", err);
        }

        Ok(())
    }

    async fn login(
        &self,
        command: &LoginCommand,
        access_token_ttl_secs: u64,
        refresh_token_ttl_secs: u64,
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

        let now = chrono::Utc::now().timestamp();

        let access_claims = TokenClaims {
            email: user.email.clone(),
            identifier: user.identifier.clone(),
            token_type: "access".to_string(),
        };
        let access_token = self.token_service.generate_token(&access_claims, access_token_ttl_secs)?;
        let access_token_expiry = now + access_token_ttl_secs as i64;

        let refresh_claims = TokenClaims {
            email: user.email.clone(),
            identifier: user.identifier.clone(),
            token_type: "refresh".to_string(),
        };
        let refresh_token = self.token_service.generate_token(&refresh_claims, refresh_token_ttl_secs)?;
        let refresh_token_expiry = now + refresh_token_ttl_secs as i64;

        Ok(LoginResult {
            access_token,
            refresh_token,
            access_token_expiry,
            refresh_token_expiry,
        })
    }

    async fn forgotten_password(
        &self,
        command: &ForgottenPasswordCommand,
    ) -> Result<ForgottenPasswordResult, AuthenticationServiceError> {
        if let Some(user) = self.repo.find_by_email(&command.email).await {
            let claims = TokenClaims {
                email: user.email.clone(),
                identifier: user.identifier.clone(),
                token_type: "access".to_string(),
            };

            if let Ok(token) = self.token_service.generate_token(&claims, 600) {
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

                if let Err(err) = self.email_sender.send_email(message) {
                    tracing::error!("Failed to send password reset email: {}", err);
                }
            }
        } else {
            tracing::warn!(
                "Password reset requested for non-existent email: {}",
                command.email
            );
        }

        Ok(ForgottenPasswordResult {})
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
        access_token_ttl_secs: u64,
        refresh_token_ttl_secs: u64,
    ) -> Result<RefreshTokenResult, AuthenticationServiceError> {
        let refresh_claims = self.token_service.validate_token(&command.refresh_token)?;

        if refresh_claims.token_type != "refresh" {
            return Err(AuthenticationServiceError::InvalidToken);
        }

        let now = chrono::Utc::now().timestamp();

        let access_claims = TokenClaims {
            email: refresh_claims.email.clone(),
            identifier: refresh_claims.identifier.clone(),
            token_type: "access".to_string(),
        };
        let access_token = self.token_service.generate_token(&access_claims, access_token_ttl_secs)?;
        let access_token_expiry = now + access_token_ttl_secs as i64;

        let new_refresh_claims = TokenClaims {
            email: refresh_claims.email.clone(),
            identifier: refresh_claims.identifier.clone(),
            token_type: "refresh".to_string(),
        };
        let new_refresh_token = self.token_service.generate_token(&new_refresh_claims, refresh_token_ttl_secs)?;
        let refresh_token_expiry = now + refresh_token_ttl_secs as i64;

        Ok(RefreshTokenResult {
            access_token,
            refresh_token: new_refresh_token,
            access_token_expiry,
            refresh_token_expiry,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ports::{
        email_sender::MockEmailSender, token_service::MockTokenService,
        user_repository::MockUserRepositoryTrait,
    };
    use sea_orm::sqlx::types::chrono::Utc;

    fn test_claims() -> TokenClaims {
        TokenClaims {
            email: "user@test.com".to_string(),
            identifier: "user-001".to_string(),
            token_type: "access".to_string(),
        }
    }

    fn test_user_model(password: &str) -> crate::models::users::Model {
        crate::models::users::Model {
            identifier: "user-001".to_string(),
            email: "user@test.com".to_string(),
            password: password.to_string(),
            first_name: Some("Test".to_string()),
            last_name: Some("User".to_string()),
            created_at: Utc::now().naive_utc().and_utc().into(),
            updated_at: None,
            is_active: true,
            role_identifier: None,
            profile_picture: None,
            username: None,
        }
    }

    fn test_create_command() -> CreateUserCommand {
        CreateUserCommand {
            email: "new@test.com".to_string(),
            password: "Password123!".to_string(),
            first_name: "New".to_string(),
            last_name: "User".to_string(),
        }
    }

    fn setup_auth_service(
        repo: MockUserRepositoryTrait,
        token_service: MockTokenService,
        email_sender: MockEmailSender,
    ) -> AuthenticationService<MockUserRepositoryTrait, MockTokenService, MockEmailSender> {
        AuthenticationService::new(repo, token_service, email_sender)
    }

    // --- create_user tests ---

    // #[tokio::test]
    //     #[ignore = "broken"]
    // async fn create_user_success() {
    //     let mut repo = MockUserRepositoryTrait::new();
    //     let token_service = MockTokenService::new();
    //     let mut email_sender = MockEmailSender::new();

    //     repo.expect_find_by_email()
    //         .returning(|_| Box::pin(async { None }));
    //     repo.expect_create_user()
    //         .returning(|_| Box::pin(async { Ok(()) }));
    //     email_sender
    //         .expect_send_email()
    //         .returning(|_| Box::pin(async { Ok(()) }));

    //     let service = setup_auth_service(repo, token_service, email_sender);
    //     let cmd = test_create_command();

    //     let result = service.create_user(&cmd).await;
    //     assert!(result.is_ok());
    // }

    #[tokio::test]
    async fn create_user_duplicate_email() {
        let mut repo = MockUserRepositoryTrait::new();
        let token_service = MockTokenService::new();
        let email_sender = MockEmailSender::new();

        let existing_user = test_user_model("hashed");
        repo.expect_find_by_email().returning(move |_| {
            let u = existing_user.clone();
            Box::pin(async move { Some(u) })
        });

        let service = setup_auth_service(repo, token_service, email_sender);
        let cmd = test_create_command();

        let result = service.create_user(&cmd).await;
        assert!(result.is_err());
    }

    // --- login tests ---

    #[tokio::test]
    async fn login_success() {
        let mut repo = MockUserRepositoryTrait::new();
        let mut token_service = MockTokenService::new();
        let email_sender = MockEmailSender::new();

        let hashed_password = bcrypt::hash("Password123!", bcrypt::DEFAULT_COST).unwrap();
        let user = test_user_model(&hashed_password);
        repo.expect_find_by_email().returning(move |_| {
            let u = user.clone();
            Box::pin(async move { Some(u) })
        });
        token_service
            .expect_generate_token()
            .returning(|_, _| Ok("jwt-token-abc".to_string()));

        let service = setup_auth_service(repo, token_service, email_sender);
        let cmd = LoginCommand {
            email: "user@test.com".to_string(),
            password: "Password123!".to_string(),
        };

        let result = service.login(&cmd, 600, 25200).await;
        assert!(result.is_ok());
        let login_result = result.unwrap();
        assert_eq!(login_result.access_token, "jwt-token-abc");
        assert_eq!(login_result.refresh_token, "jwt-token-abc");
    }

    #[tokio::test]
    async fn login_wrong_email() {
        let mut repo = MockUserRepositoryTrait::new();
        let token_service = MockTokenService::new();
        let email_sender = MockEmailSender::new();

        repo.expect_find_by_email()
            .returning(|_| Box::pin(async { None }));

        let service = setup_auth_service(repo, token_service, email_sender);
        let cmd = LoginCommand {
            email: "nobody@test.com".to_string(),
            password: "whatever".to_string(),
        };

        let result = service.login(&cmd, 600, 25200).await;
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            AuthenticationServiceError::WrongCredentials
        ));
    }

    #[tokio::test]
    async fn login_wrong_password() {
        let mut repo = MockUserRepositoryTrait::new();
        let token_service = MockTokenService::new();
        let email_sender = MockEmailSender::new();

        let hashed_password = bcrypt::hash("CorrectPassword", bcrypt::DEFAULT_COST).unwrap();
        let user = test_user_model(&hashed_password);
        repo.expect_find_by_email().returning(move |_| {
            let u = user.clone();
            Box::pin(async move { Some(u) })
        });

        let service = setup_auth_service(repo, token_service, email_sender);
        let cmd = LoginCommand {
            email: "user@test.com".to_string(),
            password: "WrongPassword".to_string(),
        };

        let result = service.login(&cmd, 600, 25200).await;
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            AuthenticationServiceError::WrongCredentials
        ));
    }

    // --- forgotten_password tests ---

    // #[tokio::test]
    // async fn forgotten_password_success() {
    //     let mut repo = MockUserRepositoryTrait::new();
    //     let mut token_service = MockTokenService::new();
    //     let mut email_sender = MockEmailSender::new();

    //     let user = test_user_model("hashed");
    //     repo.expect_find_by_email().returning(move |_| {
    //         let u = user.clone();
    //         Box::pin(async move { Some(u) })
    //     });
    //     token_service
    //         .expect_generate_token()
    //         .returning(|_, _| Ok("reset-token-xyz".to_string()));
    //     email_sender
    //         .expect_send_email()
    //         .returning(|_| Box::pin(async { Ok(()) }));

    //     let service = setup_auth_service(repo, token_service, email_sender);
    //     let cmd = ForgottenPasswordCommand {
    //         email: "user@test.com".to_string(),
    //     };

    //     let result = service.forgotten_password(&cmd).await;
    //     assert!(result.is_ok());
    //     assert_eq!(result.unwrap().token, "reset-token-xyz");
    // }

    #[tokio::test]
    async fn forgotten_password_user_not_found() {
        let mut repo = MockUserRepositoryTrait::new();
        let token_service = MockTokenService::new();
        let email_sender = MockEmailSender::new();

        repo.expect_find_by_email()
            .returning(|_| Box::pin(async { None }));

        let service = setup_auth_service(repo, token_service, email_sender);
        let cmd = ForgottenPasswordCommand {
            email: "nobody@test.com".to_string(),
        };

        let result = service.forgotten_password(&cmd).await;
        assert!(result.is_ok());
    }

    // --- set_new_password tests ---

    #[tokio::test]
    async fn set_new_password_success() {
        let mut repo = MockUserRepositoryTrait::new();
        let token_service = MockTokenService::new();
        let email_sender = MockEmailSender::new();

        let user = test_user_model("old-hash");
        repo.expect_find_by_identifier().returning(move |_| {
            let u = user.clone();
            Box::pin(async move { Some(u) })
        });
        repo.expect_update_password()
            .returning(|_, _| Box::pin(async { Ok(()) }));

        let service = setup_auth_service(repo, token_service, email_sender);
        let claims = test_claims();
        let cmd = SetNewPasswordCommand {
            password: "NewSecurePass!".to_string(),
            confirm_password: "NewSecurePass!".to_string(),
        };

        let result = service.set_new_password(&cmd, &claims).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn set_new_password_user_not_found() {
        let mut repo = MockUserRepositoryTrait::new();
        let token_service = MockTokenService::new();
        let email_sender = MockEmailSender::new();

        repo.expect_find_by_identifier()
            .returning(|_| Box::pin(async { None }));

        let service = setup_auth_service(repo, token_service, email_sender);
        let claims = test_claims();
        let cmd = SetNewPasswordCommand {
            password: "NewSecurePass!".to_string(),
            confirm_password: "NewSecurePass!".to_string(),
        };

        let result = service.set_new_password(&cmd, &claims).await;
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            AuthenticationServiceError::InvalidToken
        ));
    }

    // --- verify_account tests ---

    #[tokio::test]
    async fn verify_account_success() {
        let mut repo = MockUserRepositoryTrait::new();
        let token_service = MockTokenService::new();
        let email_sender = MockEmailSender::new();

        let user = test_user_model("hashed");
        repo.expect_find_by_identifier().returning(move |_| {
            let u = user.clone();
            Box::pin(async move { Some(u) })
        });
        repo.expect_update_account_status()
            .returning(|_| Box::pin(async { Ok(()) }));

        let service = setup_auth_service(repo, token_service, email_sender);
        let claims = test_claims();
        let cmd = VerifyAccountCommand {
            otp: "123456".to_string(),
        };

        let result = service.verify_account(&claims, &cmd).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn verify_account_user_not_found() {
        let mut repo = MockUserRepositoryTrait::new();
        let token_service = MockTokenService::new();
        let email_sender = MockEmailSender::new();

        repo.expect_find_by_identifier()
            .returning(|_| Box::pin(async { None }));

        let service = setup_auth_service(repo, token_service, email_sender);
        let claims = test_claims();
        let cmd = VerifyAccountCommand {
            otp: "123456".to_string(),
        };

        let result = service.verify_account(&claims, &cmd).await;
        assert!(result.is_err());
    }

    // --- request_refresh_token tests ---

    #[tokio::test]
    async fn request_refresh_token_success() {
        let repo = MockUserRepositoryTrait::new();
        let mut token_service = MockTokenService::new();
        let email_sender = MockEmailSender::new();

        token_service
            .expect_validate_token()
            .returning(|_| {
                Ok(TokenClaims {
                    email: "user@test.com".to_string(),
                    identifier: "user-001".to_string(),
                    token_type: "refresh".to_string(),
                })
            });
        token_service
            .expect_generate_token()
            .returning(|_, _| Ok("new-token-abc".to_string()));

        let service = setup_auth_service(repo, token_service, email_sender);
        let cmd = RefreshTokenCommand {
            refresh_token: "old-refresh-token".to_string(),
        };

        let result = service.request_refresh_token(&cmd, 600, 25200).await;
        assert!(result.is_ok());
        let refresh_result = result.unwrap();
        assert_eq!(refresh_result.access_token, "new-token-abc");
        assert_eq!(refresh_result.refresh_token, "new-token-abc");
    }
}
