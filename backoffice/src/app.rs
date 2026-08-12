use std::net::{Ipv4Addr, SocketAddrV4};

use axum::{Router, extract::DefaultBodyLimit, http::StatusCode};
use tower_http::{limit::RequestBodyLimitLayer, timeout::TimeoutLayer};

use crate::config::db::init_db_pool;
use crate::config::{
    cors::init_cors, env::load_config, logger::init_tracing, shutdown::shutdown_signal,
};
use crate::errors::app_error::AppError;
use crate::graphql::build_router as load_graphql_router;
use crate::http::routes::router::load_routes as load_http_routes;
use crate::state::AppState;

pub async fn run() -> Result<(), AppError> {
    let app_config = load_config()?;

    init_tracing(&app_config);

    let db_conn = init_db_pool(&app_config).await?;

    let app_state = AppState::new(&db_conn)?;
    let graphql_router = load_graphql_router(&app_config, app_state.clone())?;
    let http_routes = load_http_routes(app_state);

    let app = Router::new()
        .merge(graphql_router)
        .merge(http_routes)
        .layer(DefaultBodyLimit::disable())
        .layer(RequestBodyLimitLayer::new(
            app_config.body_limit_megabytes * 1024 * 1024,
        ))
        .layer(TimeoutLayer::with_status_code(
            StatusCode::REQUEST_TIMEOUT,
            app_config.requests_time_out_secs,
        ))
        .layer(tower_http::trace::TraceLayer::new_for_http())
        // .layer(CsrfLayer::new(app_config.allowed_origins.clone()))
        .layer(init_cors(&app_config));

    let ip_address = SocketAddrV4::new(Ipv4Addr::UNSPECIFIED, app_config.port);

    if app_config.is_development() {
        tracing::info!(
            "Visit GraphQL Playground at http://{}{}",
            ip_address,
            app_config.endpoint
        );
    }

    tracing::info!("Service health check at http://{}/health", ip_address,);

    let listener = tokio::net::TcpListener::bind(ip_address).await?;

    // Spawn background task runner
    tokio::spawn(crate::background::run());

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    tracing::info!("Server shutdown completed");
    Ok(())
}
