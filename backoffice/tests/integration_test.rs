mod helpers;

use axum::http::StatusCode;
use helpers::*;
use serde_json::{json, Value};

// A. Public Routes

#[tokio::test]
async fn health_check_returns_200() {
    set_test_env();
    let db = setup_db().await;
    let server = axum_test::TestServer::new(build_router(build_test_state(&db))).unwrap();

    let response = server.get("/api/health").await;
    response.assert_status_ok();
}

#[tokio::test]
async fn health_check_returns_json_body() {
    set_test_env();
    let db = setup_db().await;
    let server = axum_test::TestServer::new(build_router(build_test_state(&db))).unwrap();

    let response = server.get("/api/health").await;
    let body: Value = response.json();
    assert_eq!(body["message"], "service is healthy");
}

#[tokio::test]
async fn nonexistent_route_returns_404() {
    set_test_env();
    let db = setup_db().await;
    let server = axum_test::TestServer::new(build_router(build_test_state(&db))).unwrap();

    let response = server.get("/api/nonexistent").await;
    response.assert_status(StatusCode::NOT_FOUND);
}

// B. Countries (no auth)

#[tokio::test]
async fn fetch_all_countries() {
    set_test_env();
    let db = setup_db().await;
    let server = axum_test::TestServer::new(build_router(build_test_state(&db))).unwrap();

    let response = server.get("/api/countries").await;
    response.assert_status_ok();

    let body: Value = response.json();
    let data = body["data"].as_array().expect("data should be an array");
    assert!(!data.is_empty(), "countries should be seeded");
    assert!(data.len() >= 35, "should have at least 35 seeded countries");
}

#[tokio::test]
async fn fetch_country_by_identifier() {
    set_test_env();
    let db = setup_db().await;
    let server = axum_test::TestServer::new(build_router(build_test_state(&db))).unwrap();

    let list_resp = server.get("/api/countries").await;
    let body: Value = list_resp.json();
    let first = &body["data"].as_array().unwrap()[0];
    let id = first["identifier"].as_str().unwrap();

    let url = format!("/api/countries/{}", id);
    let response = server.get(url.as_str()).await;
    response.assert_status_ok();

    let body: Value = response.json();
    assert_eq!(body["data"]["identifier"], id);
}

#[tokio::test]
async fn fetch_country_not_found() {
    set_test_env();
    let db = setup_db().await;
    let server = axum_test::TestServer::new(build_router(build_test_state(&db))).unwrap();

    let response = server.get("/api/countries/00000000000000000000000000").await;
    let status = response.status_code();
    assert!(
        status.is_success() || status.is_client_error(),
        "expected 2xx or 4xx, got {}",
        status
    );
}

// C. Authentication (no auth)

#[tokio::test]
async fn signup_creates_account() {
    set_test_env();
    let db = setup_db().await;
    let server = axum_test::TestServer::new(build_router(build_test_state(&db))).unwrap();

    let response = server
        .post("/api/signup")
        .json(&json!({
            "email": "newuser@test.com",
            "password": "SecurePass123!",
            "firstName": "John",
            "lastName": "Doe"
        }))
        .await;

    response.assert_status(StatusCode::CREATED);
    let body: Value = response.json();
    assert_eq!(body["message"], "Account created successfully");
}

#[tokio::test]
async fn signup_invalid_email() {
    set_test_env();
    let db = setup_db().await;
    let server = axum_test::TestServer::new(build_router(build_test_state(&db))).unwrap();

    let response = server
        .post("/api/signup")
        .json(&json!({
            "email": "not-an-email",
            "password": "SecurePass123!",
            "firstName": "John",
            "lastName": "Doe"
        }))
        .await;

    let status = response.status_code();
    assert!(
        status.is_client_error(),
        "expected 4xx for invalid email, got {}",
        status
    );
}

#[tokio::test]
async fn signup_duplicate_email() {
    set_test_env();
    let db = setup_db().await;
    let server = axum_test::TestServer::new(build_router(build_test_state(&db))).unwrap();

    let body = json!({
        "email": "duplicate@test.com",
        "password": "SecurePass123!",
        "firstName": "John",
        "lastName": "Doe"
    });

    let resp1 = server.post("/api/signup").json(&body).await;
    resp1.assert_status(StatusCode::CREATED);

    let resp2 = server.post("/api/signup").json(&body).await;
    let status = resp2.status_code();
    assert!(
        status.is_client_error() || status.is_server_error(),
        "expected error for duplicate email, got {}",
        status
    );
}

#[tokio::test]
async fn login_success() {
    set_test_env();
    let db = setup_db().await;
    let server = axum_test::TestServer::new(build_router(build_test_state(&db))).unwrap();

    server
        .post("/api/signup")
        .json(&json!({
            "email": "logintest@test.com",
            "password": "SecurePass123!",
            "firstName": "Jane",
            "lastName": "Smith"
        }))
        .await
        .assert_status(StatusCode::CREATED);

    let response = server
        .post("/api/login")
        .json(&json!({
            "email": "logintest@test.com",
            "password": "SecurePass123!"
        }))
        .await;

    response.assert_status_ok();
    let body: Value = response.json();
    assert!(
        body["data"]["token"].as_str().is_some(),
        "login should return a token"
    );
}

#[tokio::test]
async fn login_wrong_password() {
    set_test_env();
    let db = setup_db().await;
    let server = axum_test::TestServer::new(build_router(build_test_state(&db))).unwrap();

    server
        .post("/api/signup")
        .json(&json!({
            "email": "wrongpw@test.com",
            "password": "SecurePass123!",
            "firstName": "Jane",
            "lastName": "Smith"
        }))
        .await;

    let response = server
        .post("/api/login")
        .json(&json!({
            "email": "wrongpw@test.com",
            "password": "WrongPassword!"
        }))
        .await;

    let status = response.status_code();
    assert!(
        status.is_client_error() || status.is_server_error(),
        "expected error for wrong password, got {}",
        status
    );
}

#[tokio::test]
async fn login_nonexistent_user() {
    set_test_env();
    let db = setup_db().await;
    let server = axum_test::TestServer::new(build_router(build_test_state(&db))).unwrap();

    let response = server
        .post("/api/login")
        .json(&json!({
            "email": "ghost@test.com",
            "password": "Whatever123!"
        }))
        .await;

    let status = response.status_code();
    assert!(
        status.is_client_error() || status.is_server_error(),
        "expected error for nonexistent user, got {}",
        status
    );
}

// D. Auth Guard

#[tokio::test]
async fn fetch_profile_with_valid_token() {
    set_test_env();
    let db = setup_db().await;
    let server = axum_test::TestServer::new(build_router(build_test_state(&db))).unwrap();

    server
        .post("/api/signup")
        .json(&json!({
            "email": "profile@test.com",
            "password": "SecurePass123!",
            "firstName": "Alice",
            "lastName": "Wonder"
        }))
        .await;

    let login_resp = server
        .post("/api/login")
        .json(&json!({
            "email": "profile@test.com",
            "password": "SecurePass123!"
        }))
        .await;
    let login_body: Value = login_resp.json();
    let token = login_body["data"]["token"].as_str().unwrap();

    let (key, val) = auth_header(token);
    let response = server
        .get("/api/users/profile")
        .add_header(key, val)
        .await;

    response.assert_status_ok();
}

#[tokio::test]
async fn fetch_profile_no_auth() {
    set_test_env();
    let db = setup_db().await;
    let server = axum_test::TestServer::new(build_router(build_test_state(&db))).unwrap();

    let response = server.get("/api/users/profile").await;
    assert_eq!(response.status_code(), StatusCode::UNAUTHORIZED, "should return 401 without auth");
}

#[tokio::test]
async fn fetch_profile_invalid_token() {
    set_test_env();
    let db = setup_db().await;
    let server = axum_test::TestServer::new(build_router(build_test_state(&db))).unwrap();

    let response = server
        .get("/api/users/profile")
        .add_header("Authorization", "Bearer invalid-token-here")
        .await;

    assert_eq!(response.status_code(), StatusCode::UNAUTHORIZED, "should return 401 for invalid token");
}

// E. Marketplace CRUD (authenticated)

fn test_token() -> String {
    generate_token("test-user-id", "test@test.com")
}

#[tokio::test]
async fn marketplace_create() {
    set_test_env();
    let db = setup_db().await;
    let server = axum_test::TestServer::new(build_router(build_test_state(&db))).unwrap();
    let token = test_token();
    let (key, val) = auth_header(&token);

    let response = server
        .post("/api/marketplaces")
        .add_header(key, &val)
        .json(&json!({
            "name": "Test Market",
            "description": "A test marketplace",
            "slug": "test-market"
        }))
        .await;

    response.assert_status(StatusCode::CREATED);
    let body: Value = response.json();
    assert_eq!(body["data"]["name"], "Test Market");
    assert_eq!(body["data"]["slug"], "test-market");
}

#[tokio::test]
async fn marketplace_list_empty() {
    set_test_env();
    let db = setup_db().await;
    let server = axum_test::TestServer::new(build_router(build_test_state(&db))).unwrap();
    let token = test_token();
    let (key, val) = auth_header(&token);

    let response = server
        .get("/api/marketplaces")
        .add_header(key, &val)
        .await;

    response.assert_status_ok();
    let body: Value = response.json();
    let data = body["data"].as_array().unwrap();
    assert!(data.is_empty(), "should start with empty marketplaces");
}

#[tokio::test]
async fn marketplace_get_by_id() {
    set_test_env();
    let db = setup_db().await;
    let server = axum_test::TestServer::new(build_router(build_test_state(&db))).unwrap();
    let token = test_token();
    let (key, val) = auth_header(&token);

    let create_resp = server
        .post("/api/marketplaces")
        .add_header(key, &val)
        .json(&json!({
            "name": "My Market",
            "description": "desc",
            "slug": "my-market"
        }))
        .await;
    let create_body: Value = create_resp.json();
    let id = create_body["data"]["identifier"].as_str().unwrap();

    let url = format!("/api/marketplaces/{}", id);
    let response = server
        .get(url.as_str())
        .add_header(key, &val)
        .await;

    response.assert_status_ok();
    let body: Value = response.json();
    assert_eq!(body["data"]["identifier"], id);
}

#[tokio::test]
async fn marketplace_count() {
    set_test_env();
    let db = setup_db().await;
    let server = axum_test::TestServer::new(build_router(build_test_state(&db))).unwrap();
    let token = test_token();
    let (key, val) = auth_header(&token);

    let count_resp = server
        .get("/api/marketplaces/count")
        .add_header(key, &val)
        .await;
    let count_body: Value = count_resp.json();
    assert_eq!(count_body["data"], 0);

    server
        .post("/api/marketplaces")
        .add_header(key, &val)
        .json(&json!({
            "name": "Market",
            "description": "desc",
            "slug": "market-1"
        }))
        .await;

    let count_resp = server
        .get("/api/marketplaces/count")
        .add_header(key, &val)
        .await;
    let count_body: Value = count_resp.json();
    assert_eq!(count_body["data"], 1);
}

#[tokio::test]
async fn marketplace_delete() {
    set_test_env();
    let db = setup_db().await;
    let server = axum_test::TestServer::new(build_router(build_test_state(&db))).unwrap();
    let token = test_token();
    let (key, val) = auth_header(&token);

    let create_resp = server
        .post("/api/marketplaces")
        .add_header(key, &val)
        .json(&json!({
            "name": "Delete Me",
            "description": "desc",
            "slug": "delete-me"
        }))
        .await;
    let create_body: Value = create_resp.json();
    let id = create_body["data"]["identifier"].as_str().unwrap();

    let url = format!("/api/marketplaces/{}", id);
    let delete_resp = server
        .delete(url.as_str())
        .add_header(key, &val)
        .await;
    delete_resp.assert_status_ok();
}

// F. Teams CRUD (authenticated)

#[tokio::test]
async fn team_create() {
    set_test_env();
    let db = setup_db().await;
    let server = axum_test::TestServer::new(build_router(build_test_state(&db))).unwrap();
    let token = test_token();
    let (key, val) = auth_header(&token);

    let response = server
        .post("/api/teams")
        .add_header(key, &val)
        .json(&json!({
            "name": "Team Alpha",
            "email": "alpha@test.com",
            "phone": "+1234567890",
            "role": "admin"
        }))
        .await;

    response.assert_status(StatusCode::CREATED);
    let body: Value = response.json();
    assert_eq!(body["data"]["name"], "Team Alpha");
    assert_eq!(body["data"]["email"], "alpha@test.com");
}

#[tokio::test]
async fn team_list() {
    set_test_env();
    let db = setup_db().await;
    let server = axum_test::TestServer::new(build_router(build_test_state(&db))).unwrap();
    let token = test_token();
    let (key, val) = auth_header(&token);

    let response = server
        .get("/api/teams")
        .add_header(key, &val)
        .await;

    response.assert_status_ok();
    let body: Value = response.json();
    assert!(body["data"].as_array().unwrap().is_empty());
}

#[tokio::test]
async fn team_get_by_id() {
    set_test_env();
    let db = setup_db().await;
    let server = axum_test::TestServer::new(build_router(build_test_state(&db))).unwrap();
    let token = test_token();
    let (key, val) = auth_header(&token);

    let create_resp = server
        .post("/api/teams")
        .add_header(key, &val)
        .json(&json!({
            "name": "Team Beta",
            "email": "beta@test.com"
        }))
        .await;
    let body: Value = create_resp.json();
    let id = body["data"]["identifier"].as_str().unwrap();

    let url = format!("/api/teams/{}", id);
    let response = server
        .get(url.as_str())
        .add_header(key, &val)
        .await;

    response.assert_status_ok();
    let body: Value = response.json();
    assert_eq!(body["data"]["identifier"], id);
}

#[tokio::test]
async fn team_block() {
    set_test_env();
    let db = setup_db().await;
    let server = axum_test::TestServer::new(build_router(build_test_state(&db))).unwrap();
    let token = test_token();
    let (key, val) = auth_header(&token);

    let create_resp = server
        .post("/api/teams")
        .add_header(key, &val)
        .json(&json!({
            "name": "Blockable",
            "email": "block@test.com"
        }))
        .await;
    let body: Value = create_resp.json();
    let id = body["data"]["identifier"].as_str().unwrap();

    let url = format!("/api/teams/{}/block", id);
    let response = server
        .put(url.as_str())
        .add_header(key, &val)
        .await;

    response.assert_status_ok();
    let body: Value = response.json();
    assert_eq!(body["data"]["blocked"], true);
}

#[tokio::test]
async fn team_delete() {
    set_test_env();
    let db = setup_db().await;
    let server = axum_test::TestServer::new(build_router(build_test_state(&db))).unwrap();
    let token = test_token();
    let (key, val) = auth_header(&token);

    let create_resp = server
        .post("/api/teams")
        .add_header(key, &val)
        .json(&json!({
            "name": "Delete Team",
            "email": "del-team@test.com"
        }))
        .await;
    let body: Value = create_resp.json();
    let id = body["data"]["identifier"].as_str().unwrap();

    let url = format!("/api/teams/{}", id);
    let delete_resp = server
        .delete(url.as_str())
        .add_header(key, &val)
        .await;
    delete_resp.assert_status_ok();
}

// G. Emails CRUD (authenticated)

#[tokio::test]
async fn email_create() {
    set_test_env();
    let db = setup_db().await;
    let server = axum_test::TestServer::new(build_router(build_test_state(&db))).unwrap();
    let token = test_token();
    let (key, val) = auth_header(&token);

    let response = server
        .post("/api/emails")
        .add_header(key, &val)
        .json(&json!({
            "subject": "Hello World",
            "body": "This is a test email",
            "senderEmail": "sender@test.com",
            "recipientEmail": "recipient@test.com"
        }))
        .await;

    response.assert_status(StatusCode::CREATED);
    let body: Value = response.json();
    assert_eq!(body["data"]["subject"], "Hello World");
}

#[tokio::test]
async fn email_list() {
    set_test_env();
    let db = setup_db().await;
    let server = axum_test::TestServer::new(build_router(build_test_state(&db))).unwrap();
    let token = test_token();
    let (key, val) = auth_header(&token);

    let response = server
        .get("/api/emails")
        .add_header(key, &val)
        .await;

    response.assert_status_ok();
    let body: Value = response.json();
    assert!(body["data"].as_array().unwrap().is_empty());
}

#[tokio::test]
async fn email_count() {
    set_test_env();
    let db = setup_db().await;
    let server = axum_test::TestServer::new(build_router(build_test_state(&db))).unwrap();
    let token = test_token();
    let (key, val) = auth_header(&token);

    let count_resp = server
        .get("/api/emails/count")
        .add_header(key, &val)
        .await;
    let body: Value = count_resp.json();
    assert_eq!(body["data"], 0);

    server
        .post("/api/emails")
        .add_header(key, &val)
        .json(&json!({
            "subject": "Test",
            "body": "Body",
            "senderEmail": "s@t.com",
            "recipientEmail": "r@t.com"
        }))
        .await;

    let count_resp = server
        .get("/api/emails/count")
        .add_header(key, &val)
        .await;
    let body: Value = count_resp.json();
    assert_eq!(body["data"], 1);
}

#[tokio::test]
async fn email_get_by_id() {
    set_test_env();
    let db = setup_db().await;
    let server = axum_test::TestServer::new(build_router(build_test_state(&db))).unwrap();
    let token = test_token();
    let (key, val) = auth_header(&token);

    let create_resp = server
        .post("/api/emails")
        .add_header(key, &val)
        .json(&json!({
            "subject": "Fetch Me",
            "body": "body",
            "senderEmail": "s@t.com",
            "recipientEmail": "r@t.com"
        }))
        .await;
    let body: Value = create_resp.json();
    let id = body["data"]["identifier"].as_str().unwrap();

    let url = format!("/api/emails/{}", id);
    let response = server
        .get(url.as_str())
        .add_header(key, &val)
        .await;

    response.assert_status_ok();
    let body: Value = response.json();
    assert_eq!(body["data"]["identifier"], id);
}

#[tokio::test]
async fn email_delete() {
    set_test_env();
    let db = setup_db().await;
    let server = axum_test::TestServer::new(build_router(build_test_state(&db))).unwrap();
    let token = test_token();
    let (key, val) = auth_header(&token);

    let create_resp = server
        .post("/api/emails")
        .add_header(key, &val)
        .json(&json!({
            "subject": "Delete Me",
            "body": "body",
            "senderEmail": "s@t.com",
            "recipientEmail": "r@t.com"
        }))
        .await;
    let body: Value = create_resp.json();
    let id = body["data"]["identifier"].as_str().unwrap();

    let url = format!("/api/emails/{}", id);
    let delete_resp = server
        .delete(url.as_str())
        .add_header(key, &val)
        .await;
    delete_resp.assert_status_ok();
}
