use crate::entities::privileged_user::PrivilegedUser;
use crate::errors::app_error::AppError;
use crate::shared::extract_env::extract_env;
use bcrypt::DEFAULT_COST;
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Postgres};
use ulid::Ulid;

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
pub async fn create_super_admin_from_env(db: &Pool<Postgres>) -> Result<(), AppError> {
    let credentials = SuperAdminCredentials::from_env().unwrap_or_default();
    let role_identifier = create_super_admin_role(&db).await?;
    let identifier = Ulid::new();
    let hashed_password = bcrypt::hash(&credentials.password, DEFAULT_COST)
        .map_err(|err| AppError::StartupError(err.to_string()))?;

    let query = r#"INSERT INTO users (identifier, role_identifier, email, password, is_active) VALUES ($1, $2, $3, $4, $5)"#;

    sqlx::query(query)
        .bind(identifier.to_string())
        .bind(&role_identifier)
        .bind(credentials.username)
        .bind(hashed_password)
        .bind(true)
        .execute(db)
        .await
        .map_err(|err| AppError::OperationFailed(err.to_string()))?;

    Ok(())
}

pub async fn super_admin_exists(db: &Pool<Postgres>) -> Result<bool, AppError> {
    let privilege_user = sqlx::query_as!(
        PrivilegedUser,
        r#"
     SELECT
            u.email,
            u.identifier,
            u.first_name,
            u.last_name,
            u.is_active,
            r.name AS role_name,
            r.description AS role_description

        FROM users u
        JOIN user_roles r ON u.role_identifier = r.identifier
        WHERE r.name = 'super_admin'
        LIMIT 1
    "#
    )
    .fetch_optional(db)
    .await
    .map_err(|e| AppError::StartupError(format!("DB error: {}", e)))?;

    Ok(privilege_user.is_some())
}

pub async fn create_super_admin_role(db: &Pool<Postgres>) -> Result<String, AppError> {
    let query = r#"
    INSERT INTO user_roles(identifier,name, description) VALUES ($1, $2, $3)
    "#;

    let identifier = Ulid::new();
    sqlx::query(query)
        .bind(&identifier.to_string())
        .bind("super_admin")
        .bind("Unrestricted access to all resources")
        .execute(db)
        .await
        .map_err(|err| {
            let err_msg = format!("failed to create super_admin role due to {}", err);
            AppError::StartupError(err_msg)
        })?;
    Ok(identifier.to_string())
}
