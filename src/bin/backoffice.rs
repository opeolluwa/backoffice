use std::{
    net::{Ipv4Addr, SocketAddr, SocketAddrV4},
    time::Duration,
};

use axum::extract::DefaultBodyLimit;

use backoffice_lib::{
    config::{
        app::{create_cors_layer, shutdown_signal},
        app_config::AppConfig,
        database::AppDatabase,
        filesystem::AppFileSystem,
        logger::AppLogger,
    },
    errors, routes,
};

use errors::app_error::AppError;
use routes::router::load_routes;

use tower_http::{limit::RequestBodyLimitLayer, timeout::TimeoutLayer};

#[tokio::main]
async fn main() -> Result<(), AppError> {
    let config = AppConfig::from_env()?;
    let database_pool = AppDatabase::init_pool().await?;
    let body_limit_bytes = config.body_limit_mb * 1024 * 1024;
    let time_out_duration = Duration::from_secs(10);

    AppLogger::init();
    AppFileSystem::init(&config)?;

    let app = load_routes(database_pool)
        .layer(DefaultBodyLimit::disable())
        .layer(RequestBodyLimitLayer::new(body_limit_bytes))
        .layer(TimeoutLayer::new(time_out_duration))
        .layer(tower_http::trace::TraceLayer::new_for_http())
        .layer(create_cors_layer(&config));

    let ip_address = SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::UNSPECIFIED, config.port));
    tracing::info!("Application listening on http://{ip_address}");
    let listener = tokio::net::TcpListener::bind(ip_address).await?;
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    tracing::info!("Server shutdown completed");
    Ok(())
}
