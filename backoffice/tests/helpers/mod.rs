use std::time::Duration;

use sea_orm::DatabaseConnection;
use secrecy::SecretString;

use backoffice_api::state::{AppState, ServicesState};
use backoffice_config::env::{AppConfig, Environment};
use backoffice_infra::database::repositories::base::Repository;
use backoffice_infra::database::repositories::{
    country_repository::CountryRepository, email_repository::EmailRepository,
    invitation_repository::InvitationRepository, marketplace_repository::MarketplaceRepository,
    product_repository::ProductRepository, team_repository::TeamRepository,
    upload_repository::UploadRepository, user_repository::UserRepository,
};
use backoffice_infra::imagekit::ImagekitClient;
use backoffice_infra::jwt::JwtTokenService;
use backoffice_infra::mailer::zepto_mailer::ZeptoMail;
use migration::{Migrator, MigratorTrait};

const TEST_JWT_SECRET: &str = "test-secret-key-for-integration-tests-12345";

pub fn set_test_env() {
    // SAFETY: tests run single-threaded within each test function;
    // env vars are set once before any async work begins.
    unsafe {
        std::env::set_var("JWT_SIGNING_KEY", TEST_JWT_SECRET);
    }
}

pub fn test_config() -> AppConfig {
    AppConfig {
        port: 3000,
        environment: Environment::Test,
        body_limit_bytes: 10 * 1024 * 1024,
        upload_path: "/tmp/test-upload".into(),
        export_path: "/tmp/test-export".into(),
        allowed_origins: vec!["*".into()],
        email_api_key: "test-email-key".into(),
        email_api_user: "test-email-user".into(),
        database_url: SecretString::new("sqlite::memory:".into()),
        max_db_connections: 5,
        endpoint: "/graphql".into(),
        depth_limit: Some(100),
        complexity_limit: Some(1000),
        requests_time_out: Duration::from_secs(10),
        imagekit_public_key: "test-imagekit-public".into(),
        imagekit_private_key: "test-imagekit-private".into(),
    }
}

pub async fn setup_db() -> DatabaseConnection {
    let db = sea_orm::Database::connect("sqlite::memory:")
        .await
        .expect("Failed to connect to in-memory SQLite");
    Migrator::up(&db, None)
        .await
        .expect("Failed to run migrations");
    db
}

pub fn build_test_state(db: &DatabaseConnection) -> AppState {
    let user_repository = UserRepository::init(db);
    let country_repository = CountryRepository::init(db);
    let marketplace_repository = MarketplaceRepository::init(db);
    let product_repository = ProductRepository::init(db);
    let team_repository = TeamRepository::init(db);
    let email_repository = EmailRepository::init(db);
    let invitation_repository = InvitationRepository::init(db);
    let upload_repository = UploadRepository::init(db);

    let email_client = ZeptoMail::new("test-api-key");
    let imagekit_client = ImagekitClient::new(
        &SecretString::from("test-public-key"),
        &SecretString::from("test-private-key"),
    )
    .expect("Failed to create ImagekitClient");

    let token_service = JwtTokenService::new();
    let user_service = backoffice_domain::services::user::UserService::new(user_repository.clone());
    let auth_service = backoffice_domain::services::auth::AuthenticationService::new(
        user_repository,
        token_service,
        email_client,
    );
    let country_service =
        backoffice_domain::services::country::CountryService::new(country_repository);
    let marketplace_service =
        backoffice_domain::services::marketplace::MarketplaceService::new(marketplace_repository);
    let product_service =
        backoffice_domain::services::product::ProductService::new(product_repository);
    let team_service = backoffice_domain::services::team::TeamService::new(team_repository);
    let root_service = backoffice_domain::services::root::RootService::init();
    let emails_service = backoffice_domain::services::emails::EmailsService::new(email_repository);
    let invitation_service =
        backoffice_domain::services::invitation::InvitationService::new(invitation_repository);
    let upload_service = backoffice_domain::services::upload::UploadsService::new(
        upload_repository,
        imagekit_client,
    );

    let services = ServicesState {
        user_service,
        root_service,
        auth_service,
        marketplace_service,
        product_service,
        country_service,
        team_service,
        emails_service,
        invitation_service,
        upload_service,
    };

    AppState {
        services,
        database_connection: db.clone(),
        app_config: test_config(),
    }
}

pub fn build_router(state: AppState) -> axum::Router {
    backoffice_api::load_http_routes(state)
}

pub fn generate_token(identifier: &str, email: &str) -> String {
    backoffice_api::http::dto::jwt::JwtCredentials::new(email, identifier)
        .generate_token(Duration::from_secs(3600))
        .expect("Failed to generate test token")
}

pub fn auth_header(token: &str) -> (&str, String) {
    ("Authorization", format!("Bearer {}", token))
}
