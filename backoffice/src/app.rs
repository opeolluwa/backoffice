use std::{
    net::{Ipv4Addr, SocketAddr, SocketAddrV4},
    time::Duration,
};

use crate::{
    api::{load_graphql_router, load_http_routes, state::AppState},
    config::{
        app::{create_cors_layer, shutdown_signal},
        app_config::load_config,
        logger::AppLogger,
    },
    errors,
    fs::filesystem::AppFileSystem,
    infrastructure::database::connection::init_db_pool,
};
use axum::{Router, extract::DefaultBodyLimit, http::StatusCode};
use errors::app_error::AppError;
use tower_http::{limit::RequestBodyLimitLayer, timeout::TimeoutLayer};

pub async fn run() -> Result<(), AppError> {
    let app_config = load_config()?;
    let db_conn = init_db_pool().await?;
    let body_limit_bytes = app_config.body_limit_mb * 1024 * 1024;
    let time_out_duration = Duration::from_secs(10);

    AppLogger::init();
    AppFileSystem::init(&app_config)?;

    let app_state = AppState::new(&db_conn)?;

    let graphql_router = load_graphql_router(db_conn, &app_config, app_state.clone())?;
    let http_routes = load_http_routes(app_state);

    let app = Router::new()
        .merge(graphql_router)
        .merge(http_routes)
        .layer(DefaultBodyLimit::disable())
        .layer(RequestBodyLimitLayer::new(body_limit_bytes))
        .layer(TimeoutLayer::with_status_code(
            StatusCode::REQUEST_TIMEOUT,
            time_out_duration,
        ))
        .layer(tower_http::trace::TraceLayer::new_for_http())
        .layer(create_cors_layer(&app_config));

    let ip_address = SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::UNSPECIFIED, app_config.port));
    tracing::info!(
        "Visit GraphQL Playground at http://{}{}",
        ip_address,
        app_config.endpoint
    );
    tracing::info!("Service health check at http://{}/health", ip_address,);

    let listener = tokio::net::TcpListener::bind(ip_address).await?;
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    tracing::info!("Server shutdown completed");
    Ok(())
}
