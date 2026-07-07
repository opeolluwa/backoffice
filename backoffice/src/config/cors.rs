use axum::http::{HeaderValue, Method, header};
use tower_http::cors::{Any, CorsLayer};

use crate::config::{self, env::AppConfig};

pub fn init_cors(config: &AppConfig) -> CorsLayer {
    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_headers([header::CONTENT_TYPE, header::AUTHORIZATION]);

    if config.environment == config::env::Environment::Production {
        if config.allowed_origins.len() == 1 && config.allowed_origins[0] == "*" {
            cors.allow_origin(Any)
        } else {
            let origins = config
                .allowed_origins
                .iter()
                .map(|origin| origin.parse::<HeaderValue>())
                .collect::<Result<Vec<_>, _>>()
                .expect("Invalid CORS origin in configuration");

            cors.allow_origin(origins)
        }
    } else {
        cors.allow_origin(Any)
    }
}
