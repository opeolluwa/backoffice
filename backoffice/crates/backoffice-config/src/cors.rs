use axum::http::{HeaderValue, Method, header};
use tower_http::cors::{Any, CorsLayer};

use crate::env::{self, AppConfig};

pub fn init_cors(config: &AppConfig) -> CorsLayer {
    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_headers([header::CONTENT_TYPE, header::AUTHORIZATION]);

    if config.environment == env::Environment::Production {
        let has_wildcard = config
            .allowed_origins
            .iter()
            .any(|o| o == "*" || o == "Any");

        if has_wildcard {
            tracing::warn!(
                "CORS: wildcard origin is not allowed in production; \
                 restricting to same-origin only"
            );
            cors
        } else {
            let origins = config
                .allowed_origins
                .iter()
                .filter_map(|origin| {
                    origin.parse::<HeaderValue>().ok()
                })
                .collect::<Vec<_>>();

            if origins.is_empty() {
                tracing::warn!("CORS: no valid origins configured; denying all cross-origin requests");
                cors
            } else {
                cors.allow_origin(origins)
            }
        }
    } else {
        cors.allow_origin(Any)
    }
}
