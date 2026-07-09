use std::time::Duration;

#[derive(Debug, Clone)]
pub struct AppConfig {
    pub port: u16,
    pub body_limit_bytes: usize,
    pub request_timeout: Duration,
}

impl AppConfig {
    fn env_or(key: &str, default: &str) -> String {
        std::env::var(key).unwrap_or_else(|_| default.to_string())
    }
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            port: 4000,
            body_limit_bytes: 2 * 1024 * 1024,
            request_timeout: Duration::from_secs(30),
        }
    }
}

pub fn load_config() -> AppConfig {
    AppConfig {
        port: AppConfig::env_or("BACKOFFICE_CONSOLE_PORT", "4000")
            .parse()
            .unwrap_or(4000),
        body_limit_bytes: AppConfig::env_or("BACKOFFICE_CONSOLE_BODY_LIMIT", "2097152")
            .parse()
            .unwrap_or(2 * 1024 * 1024),
        request_timeout: Duration::from_secs(
            AppConfig::env_or("BACKOFFICE_CONSOLE_TIMEOUT_SECS", "30")
                .parse()
                .unwrap_or(30),
        ),
    }
}
