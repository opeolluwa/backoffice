use crate::adapters::requests::auth::CreateUserRequest;
use crate::errors::app_error::AppError;
use crate::services::auth_service::{AuthenticationService, AuthenticationServiceTrait};
use crate::shared::extract_env::extract_env;
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Postgres};

#[derive(Serialize, Deserialize)]
struct SuperAdminCredentials {
    username: String,
    password: String,
}

impl Default for SuperAdminCredentials {
    fn default() -> Self {
        SuperAdminCredentials {
            username: "admin@backoffice.dev".to_string(),
            password: "SuperAdminCredentials".to_string(),
        }
    }
}

trait FromEnv {
    fn from_env() -> Result<Self, AppError>
    where
        Self: Sized;
}

impl FromEnv for SuperAdminCredentials {
    fn from_env() -> Result<Self, AppError> {
        let username = extract_env("BACKOFFICE_SUPER_ADMIN_USER")?;
        let password = extract_env("BACKOFFICE_SUPER_ADMIN_PASS")?;

        Ok(Self { username, password })
    }
}
pub async fn super_admin_from_env(db: &Pool<Postgres>) -> Result<(), AppError> {
    let credentials = SuperAdminCredentials::from_env().unwrap_or_default();
    let admin_user = CreateUserRequest {
        email: credentials.username,
        password: credentials.password,
        first_name: "".to_string(),
        last_name: "".to_string(),
    };

    let auth_service = AuthenticationService::init(db);
    auth_service
        .create_account(&admin_user)
        .await
        .map_err(|err| AppError::OperationFailed(err.to_string()))
}

pub async fn check_super_admin() -> Result<bool, AppError> {
    Ok(true)
}
