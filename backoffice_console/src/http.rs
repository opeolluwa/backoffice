use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4};

use axum::{Router, extract::DefaultBodyLimit, http::StatusCode};
use errors::app_error::AppError;
use tower_http::{limit::RequestBodyLimitLayer, timeout::TimeoutLayer};

use crate::{
    api::{load_graphql_router, load_http_routes, state::AppState},
    config::{cors::init_cors, env::load_config, logger::init_tracing, shutdown::shutdown_signal},
    errors,
    infrastructure::database::connection::init_db_pool,
};

pub async fn run() -> Result<(), AppError> {
    init_tracing();

    let app_config = load_config()?;
    let db_conn = init_db_pool().await?;

    let app_state = AppState::new(&db_conn)?;
    let graphql_router = load_graphql_router(db_conn, &app_config, app_state.clone())?;
    let http_routes = load_http_routes(app_state);

    let app = Router::new()
        .merge(graphql_router)
        .merge(http_routes)
        .layer(DefaultBodyLimit::disable())
        .layer(RequestBodyLimitLayer::new(app_config.body_limit_bytes))
        .layer(TimeoutLayer::with_status_code(
            StatusCode::REQUEST_TIMEOUT,
            app_config.requests_time_out,
        ))
        .layer(tower_http::trace::TraceLayer::new_for_http())
        .layer(init_cors(&app_config));

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

