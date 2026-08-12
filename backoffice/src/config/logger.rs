use crate::config::env::{AppConfig, Environment};

pub struct AppLogger {}

pub fn init_tracing(app_config: &AppConfig) {
    let log_level = match &app_config.environment {
        Environment::Development | Environment::Test => tracing::Level::DEBUG,
        Environment::Production => tracing::Level::INFO,
    };
    tracing_subscriber::fmt()
        .with_max_level(log_level)
        .with_target(false)
        .compact()
        .init();

    tracing::info!("Logger initialized");
}
