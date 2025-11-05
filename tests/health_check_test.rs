

use axum_test::TestServer;
use sqlx::{PgPool};

#[sqlx::test]
async fn test_health_check(pool: PgPool) {

    let app = backoffice_lib::routes::router::load_routes(pool);
    let server = TestServer::new(app).unwrap();

    let response = server
        .get("/health")
        .await;

    response.assert_status_ok();
}
