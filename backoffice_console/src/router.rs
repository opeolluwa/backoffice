use axum::routing::{Router, get, post};
use tower_http::services::{ServeDir, ServeFile};

use crate::handler;

pub fn router() -> Router {
    let serve_dir = ServeDir::new("assets").not_found_service(ServeFile::new("assets/index.html"));
    Router::new()
        .route("/health", get(async || "Hello, World!"))
        .route("/api/create-module", post(handler::create_module))
        .fallback_service(serve_dir)
}
