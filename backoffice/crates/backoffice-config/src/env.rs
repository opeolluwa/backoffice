use std::{str::FromStr, time::Duration};

use dotenv::dotenv;
use secrecy::SecretString;
use serde::{Deserialize, Serialize};

use backoffice_domain::{errors::app_error::AppError, shared::extract_env::extract_env};

#[derive(Debug, Clone, Deserialize)]
pub struct AppConfig {
    // Server
    #[serde(default = "default_port")]
    pub port: u16,

    #[serde(default = "default_environment")]
    pub environment: Environment,

    #[serde(default = "default_body_limit_bytes")]
    pub body_limit_bytes: usize,

    // Storage
    #[serde(default = "default_upload_path")]
    pub upload_path: String,

    #[serde(default = "default_export_path")]
    pub export_path: String,

    // CORS
    #[serde(default = "default_allowed_origins")]
    pub allowed_origins: Vec<String>,

    // Email
    pub email_api_key: String,
    pub email_api_user: String,

    // Database
    pub database_url: SecretString,
    pub max_db_connections: u32,

    // GraphQL
    #[serde(default = "default_graphql_endpoint")]
    pub endpoint: String,

    #[serde(default = "default_graphql_depth_limit")]
    pub depth_limit: Option<usize>,

    #[serde(default = "default_graphql_complexity_limit")]
    pub complexity_limit: Option<usize>,

    #[serde(default = "default_requests_time_out")]
    pub requests_time_out: Duration,

    // imagekit
    pub imagekit_public_key: SecretString,
    pub imagekit_private_key: SecretString,
}

impl AppConfig {
    pub fn from_env() -> Result<Self, AppError> {
        dotenv().ok();

        tracing::info!("Loading application configuration...");

        let database_url = extract_env::<String>("DATABASE_URL")?;
        let database_url = database_url.into_boxed_str();

        let requests_time_out = extract_env::<u64>("REQUESTS_TIME_OUT")
            .unwrap_or_else(|_| default_requests_time_out().as_secs());

        Ok(Self {
            // Server
            port: extract_env("PORT").unwrap_or_else(|_| default_port()),
            environment: extract_env::<Environment>("ENVIRONMENT")
                .unwrap_or_else(|_| default_environment()),
            body_limit_bytes: extract_env("BODY_LIMIT_BYTES")
                .unwrap_or_else(|_| default_body_limit_bytes()),

            // Storage
            upload_path: extract_env("UPLOAD_PATH").unwrap_or_else(|_| default_upload_path()),
            export_path: extract_env("EXPORT_PATH").unwrap_or_else(|_| default_export_path()),

            // CORS
            allowed_origins: extract_env::<String>("ALLOWED_ORIGINS")
                .unwrap_or_else(|_| "*".into())
                .split(',')
                .map(str::trim)
                .filter(|s| !s.is_empty())
                .map(ToOwned::to_owned)
                .collect(),

            // Email
            email_api_user: extract_env("ZEPTO_EMAIL_API_USER")?,
            email_api_key: extract_env("ZEPTO_EMAIL_API_KEY")?,

            // Database
            database_url: SecretString::new(database_url),
            max_db_connections: extract_env("MAX_DB_CONNECTIONS")?,

            // GraphQL
            endpoint: extract_env("ENDPOINT").unwrap_or_else(|_| default_graphql_endpoint()),
            depth_limit: extract_env::<usize>("DEPTH_LIMIT").ok(),
            complexity_limit: extract_env::<usize>("COMPLEXITY_LIMIT").ok(),

            requests_time_out: Duration::from_secs(requests_time_out),

            imagekit_private_key: SecretString::from(extract_env::<String>(
                "IMAGEKIT_PRIVATE_KEY",
            )?),

            imagekit_public_key: SecretString::from(extract_env::<String>("IMAGEKIT_PUBLIC_KEY")?),
        })
    }

    pub fn current_env() -> Environment {
        match std::env::var("ENVIRONMENT")
            .unwrap_or_default()
            .to_lowercase()
            .as_str()
        {
            "development" | "dev" => Environment::Development,
            "production" | "prod" => Environment::Production,
            "test" | "testing" => Environment::Test,
            _ => Environment::Development,
        }
    }

    pub fn is_production(&self) -> bool {
        matches!(self.environment, Environment::Production)
    }

    pub fn is_test(&self) -> bool {
        matches!(self.environment, Environment::Test)
    }

    pub fn is_development(&self) -> bool {
        matches!(self.environment, Environment::Development)
    }
}

pub fn load_config() -> Result<AppConfig, AppError> {
    AppConfig::from_env()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Environment {
    Development,
    Production,
    Test,
}

impl FromStr for Environment {
    type Err = String;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value.to_lowercase().as_str() {
            "development" | "dev" => Ok(Self::Development),
            "production" | "prod" => Ok(Self::Production),
            "test" | "testing" => Ok(Self::Test),
            _ => Err(format!("Invalid environment: {value}")),
        }
    }
}

fn default_port() -> u16 {
    8080
}

fn default_environment() -> Environment {
    Environment::Development
}

fn default_body_limit_bytes() -> usize {
    10 * 1024 * 1024 // 10 MiB
}

fn default_upload_path() -> String {
    "/tmp/upload".into()
}

fn default_export_path() -> String {
    "/tmp/export".into()
}

fn default_allowed_origins() -> Vec<String> {
    vec!["*".into()]
}

fn default_graphql_endpoint() -> String {
    "/graphql".into()
}

fn default_graphql_depth_limit() -> Option<usize> {
    Some(100)
}

fn default_graphql_complexity_limit() -> Option<usize> {
    Some(1000)
}

fn default_requests_time_out() -> Duration {
    Duration::from_secs(10)
}
