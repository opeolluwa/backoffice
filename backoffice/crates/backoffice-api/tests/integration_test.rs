use std::sync::Arc;

use axum::{Json, Router, extract::State, http::StatusCode, routing::get};
use serde_json::{Value, json};

use backoffice_domain::services::root::{RootService, RootServiceTrait};

#[derive(Clone)]
struct TestState {
    root_service: Arc<RootService>,
}

async fn health_check_handler(
    State(state): State<Arc<TestState>>,
) -> Result<Json<Value>, StatusCode> {
    state
        .root_service
        .health_check()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let body = json!({
        "message": "service is healthy"
    });
    Ok(Json(body))
}

fn test_router() -> Router {
    let state = Arc::new(TestState {
        root_service: Arc::new(RootService::init()),
    });
    Router::new()
        .route("/api/health", get(health_check_handler))
        .with_state(state)
}

#[tokio::test]
async fn health_check_returns_200() {
    let server = axum_test::TestServer::new(test_router()).unwrap();
    let response = server.get("/api/health").await;
    response.assert_status_ok();
}

#[tokio::test]
async fn health_check_returns_json() {
    let server = axum_test::TestServer::new(test_router()).unwrap();
    let response = server.get("/api/health").await;
    let body: Value = response.json();
    assert_eq!(body["message"], "service is healthy");
}

#[tokio::test]
async fn non_existent_route_returns_404() {
    let server = axum_test::TestServer::new(test_router()).unwrap();
    let response = server.get("/api/nonexistent").await;
    response.assert_status(StatusCode::NOT_FOUND);
}
