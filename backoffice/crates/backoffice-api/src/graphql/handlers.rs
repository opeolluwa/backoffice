use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::{
    Router,
    extract::State,
    response::{self, IntoResponse},
    routing::get,
};
use seaography::async_graphql::http::{GraphQLPlaygroundConfig, playground_source};

use backoffice_config::env::AppConfig;
use backoffice_domain::errors::app_error::AppError;

use super::query_root;
use crate::state::{AppState, GraphQlState};

pub async fn graphql_playground(
    State(GraphQlState { endpoint, .. }): State<GraphQlState>,
) -> impl IntoResponse {
    response::Html(playground_source(GraphQLPlaygroundConfig::new(&endpoint)))
}

pub async fn graphql_handler(
    State(GraphQlState { schema, .. }): State<GraphQlState>,
    req: GraphQLRequest,
) -> GraphQLResponse {
    let req = req.into_inner();
    schema.execute(req).await.into()
}

pub fn build_router(app_config: &AppConfig, app_state: AppState) -> Result<Router, AppError> {
    let schema = query_root::schema(
        app_config.depth_limit,
        app_config.complexity_limit,
        app_state,
    )
    .map_err(|err| AppError::GraphQLError(err.to_string()))?;

    let state = GraphQlState {
        schema,
        endpoint: app_config.endpoint.clone(),
    };

    Ok(Router::new()
        .route(
            &app_config.endpoint,
            get(graphql_playground).post(graphql_handler),
        )
        .with_state(state))
}
