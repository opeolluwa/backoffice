use std::env;

use dotenv::dotenv;
use tower_http::cors::AllowOrigin;

use crate::errors::app_error::AppError;
use crate::shared::extract_env::extract_env;

extern crate dotenv;

#[derive(Debug)]
pub struct AppConfig {
    pub database_url: String,
    pub max_db_connections: u32,
    pub body_limit_mb: usize,
    pub upload_path: String,
    pub export_path: String,
    pub port: u16,
    pub environment: String,
    pub allowed_origins: AllowOrigin,
    pub email_api_key: String,
    pub email_api_user: String,

    // GraphQL / API settings
    pub endpoint: String,
    pub depth_limit: Option<usize>,
    pub complexity_limit: Option<usize>,
}

impl AppConfig {
    pub fn from_env() -> Result<Self, AppError> {
        dotenv().ok();

        let port = extract_env::<u16>("PORT")?;

        let max_db_connections = extract_env::<u32>("MAX_DB_CONNECTIONS")?;

        let body_limit_mb = extract_env::<usize>("BODY_LIMIT_MB")?;

        let export_path = extract_env("EXPORT_PATH").unwrap_or_else(|_| "/tmp/export".to_string());
        let upload_path = extract_env("UPLOAD_PATH").unwrap_or_else(|_| "/tmp/upload".to_string());

        let environment = extract_env("ENVIRONMENT")?;

        // Parse allowed origins (comma-separated list)
        let allowed_origins_str =
            extract_env("ALLOWED_ORIGINS").unwrap_or_else(|_| "*".to_string());
        let allowed_origins = if allowed_origins_str == "*" {
            AllowOrigin::any()
        } else {
            let origins: Vec<_> = allowed_origins_str
                .split(',')
                .map(|s| s.trim().to_string())
                .collect();
            AllowOrigin::list(origins.into_iter().map(|s| s.parse().unwrap()))
        };

        tracing::info!("App Config loaded!");

        let email_api_user = extract_env("ZEPTO_EMAIL_API_USER")?;
        let email_api_key = extract_env("ZEPTO_EMAIL_API_KEY")?;

        let database_url = extract_env("DATABASE_URL")?;

        let endpoint = env::var("ENDPOINT").unwrap_or_else(|_| "/graphql".into());

        let depth_limit = env::var("DEPTH_LIMIT")
            .ok()
            .map(|v| v.parse().unwrap_or_else(|_| 100));

        let complexity_limit = env::var("COMPLEXITY_LIMIT")
            .ok()
            .map(|v| v.parse().unwrap_or_else(|_| 1000));

        Ok(Self {
            database_url,
            max_db_connections,
            body_limit_mb,
            upload_path,
            export_path,
            port,
            environment,
            allowed_origins,
            email_api_key,
            email_api_user,
            endpoint,
            depth_limit,
            complexity_limit,
        })
    }
}
