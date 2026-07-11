use axum::routing::{Router, get};
use tower_http::services::{ServeDir, ServeFile};

pub fn router() -> Router {
    let serve_dir = ServeDir::new("assets").not_found_service(ServeFile::new("assets/index.html"));
    Router::new()
        .route("/health", get(async || "Hello, World!"))
        .fallback_service(serve_dir)
}
