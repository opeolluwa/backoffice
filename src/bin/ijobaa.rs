use axum::extract::DefaultBodyLimit;
use backoffice_lib::config::app::{create_cors_layer, shutdown_signal};
use backoffice_lib::config::app_config::AppConfig;
use backoffice_lib::config::filesystem::AppFileSystem;
use backoffice_lib::config::logger::AppLogger;
use backoffice_lib::{errors, routes, scripts, shared};
use errors::app_error::AppError;
use routes::router::load_routes;
use shared::extract_env::extract_env;
use sqlx::migrate::Migrator;
use sqlx::postgres::PgPoolOptions;
use std::time::Duration;
use std::{
    net::{Ipv4Addr, SocketAddr, SocketAddrV4},
    path::Path,
};
use tower_http::{limit::RequestBodyLimitLayer, timeout::TimeoutLayer};

#[tokio::main]
async fn main() -> Result<(), AppError> {
    AppLogger::init();
    tracing::info!("Logger initialized");

    let config = AppConfig::from_env()?;
    AppFileSystem::init(&config)?;
    tracing::info!("App Config loaded!, {config:#?}");

    let database_url = extract_env::<String>("DATABASE_URL")?;

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .map_err(|err| AppError::StartupError(err.to_string()))?;
    log::info!("Database initialized");

    let migrator = Migrator::new(Path::new("migrations"))
        .await
        .map_err(|err| AppError::StartupError(err.to_string()))?;
    migrator
        .run(&pool)
        .await
        .map_err(|err| AppError::StartupError(err.to_string()))?;

    if !scripts::super_admin::super_admin_exists(&pool).await? {
        scripts::super_admin::create_super_admin_from_env(&pool).await?;
    }

    let body_limit_bytes = config.body_limit_mb * 1024 * 1024;

    let app = load_routes(pool)
        .layer(DefaultBodyLimit::disable())
        .layer(RequestBodyLimitLayer::new(body_limit_bytes))
        .layer(TimeoutLayer::new(Duration::from_secs(10)))
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
