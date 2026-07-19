use std::net::{Ipv4Addr, SocketAddrV4};

use axum::{extract::DefaultBodyLimit, http::StatusCode};
use tower_http::{limit::RequestBodyLimitLayer, timeout::TimeoutLayer};

use crate::{
    config::{cors::init_cors, env::load_config, logger::init_tracing, shutdown::shutdown_signal},
    errors::StartupError,
    router::router,
};

pub async fn run() -> Result<(), StartupError> {
    init_tracing();

    let cfg = load_config();

    let app = router()
        .layer(DefaultBodyLimit::disable())
        .layer(RequestBodyLimitLayer::new(cfg.body_limit_bytes))
        .layer(TimeoutLayer::with_status_code(
            StatusCode::REQUEST_TIMEOUT,
            cfg.request_timeout,
        ))
        .layer(init_cors());

    let addr = SocketAddrV4::new(Ipv4Addr::UNSPECIFIED, cfg.port);
    tracing::info!("Server listening on http://{}/health", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    tracing::info!("Server shutdown completed");
    Ok(())
}
