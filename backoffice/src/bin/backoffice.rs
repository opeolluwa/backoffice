use std::{
    net::{Ipv4Addr, SocketAddr, SocketAddrV4},
    time::Duration,
};

use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::{
    Router,
    extract::{DefaultBodyLimit, State},
    http::StatusCode,
    response::{self, IntoResponse},
    routing::get,
};
use backoffice_lib::{
    config::{
        app::{create_cors_layer, shutdown_signal},
        app_config::AppConfig,
        database::AppDatabase,
        logger::AppLogger,
    },
    errors,
    fs::filesystem::AppFileSystem,
    routes::router::load_routes,
    states::GraphQlState,
};
use errors::app_error::AppError;
use seaography::async_graphql::http::{GraphQLPlaygroundConfig, playground_source};
use tower_http::{limit::RequestBodyLimitLayer, timeout::TimeoutLayer};

async fn graphql_playground(
    State(GraphQlState { endpoint, .. }): State<GraphQlState>,
) -> impl IntoResponse {
    response::Html(playground_source(GraphQLPlaygroundConfig::new(&endpoint)))
}

async fn graphql_handler(
    State(GraphQlState { schema, .. }): State<GraphQlState>,
    req: GraphQLRequest,
) -> GraphQLResponse {
    let req = req.into_inner();
    schema.execute(req).await.into()
}

#[tokio::main]
async fn main() -> Result<(), AppError> {
    let app_config = AppConfig::from_env()?;
    let db_conn = AppDatabase::init_pool().await?;
    let body_limit_bytes = app_config.body_limit_mb * 1024 * 1024;
    let time_out_duration = Duration::from_secs(10);

    AppLogger::init();
    AppFileSystem::init(&app_config)?;

    let schema =
        backoffice_lib::query_root::schema(db_conn.clone(), Some(100), app_config.complexity_limit)
            .map_err(|err| AppError::GraphQLError(err.to_string()))?;

    let graphql_state = GraphQlState {
        schema,
        endpoint: app_config.endpoint.clone(),
    };

    let http_routes = load_routes(db_conn, &app_config);

    let graphql_router = Router::new()
        .route(
            &app_config.endpoint,
            get(graphql_playground).post(graphql_handler),
        )
        .with_state(graphql_state);

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
