use std::sync::Arc;

use async_graphql::dynamic::Schema;
use axum::extract::FromRef;
use backoffice_email_client::zepto_mailer::ZeptoMail;
use sea_orm::DatabaseConnection;
use seaography::async_graphql;

use crate::{
    config::app_config::AppConfig,
    domain::services::{
        admin::AdminService, auth::AuthenticationService, country::CountryService,
        marketplace::MarketplaceService, product::ProductService, root::RootService,
        team::TeamService, user::UserService,
    },
    errors::app_error::AppError,
    infrastructure::database::repositories::{
        base::Repository, country_repository::CountryRepository, email_repository::EmailRepository,
        marketplace_repository::MarketplaceRepository, product_repository::ProductRepository,
        team_repository::TeamRepository, upload_repository::UploadRepository,
        user_repository::UserRepository,
    },
};

#[derive(Clone)]
pub struct ServicesState {
    pub user_service: Arc<UserService<UserRepository>>,
    pub root_service: Arc<RootService>,
    pub auth_service: Arc<AuthenticationService<UserRepository>>,
    pub marketplace_service: Arc<MarketplaceService<MarketplaceRepository>>,
    pub product_service: Arc<ProductService<ProductRepository>>,
    pub country_service: Arc<CountryService<CountryRepository>>,
    pub team_service: Arc<TeamService<TeamRepository>>,
}

impl FromRef<ServicesState> for Arc<UserService<UserRepository>> {
    fn from_ref(input: &ServicesState) -> Self {
        Arc::clone(&input.user_service)
    }
}

impl FromRef<ServicesState> for Arc<RootService> {
    fn from_ref(input: &ServicesState) -> Self {
        Arc::clone(&input.root_service)
    }
}

impl FromRef<ServicesState> for Arc<AuthenticationService<UserRepository>> {
    fn from_ref(input: &ServicesState) -> Self {
        Arc::clone(&input.auth_service)
    }
}

impl FromRef<ServicesState> for Arc<MarketplaceService<MarketplaceRepository>> {
    fn from_ref(input: &ServicesState) -> Self {
        Arc::clone(&input.marketplace_service)
    }
}

impl FromRef<ServicesState> for Arc<ProductService<ProductRepository>> {
    fn from_ref(input: &ServicesState) -> Self {
        Arc::clone(&input.product_service)
    }
}

impl FromRef<ServicesState> for Arc<CountryService<CountryRepository>> {
    fn from_ref(input: &ServicesState) -> Self {
        Arc::clone(&input.country_service)
    }
}

impl FromRef<ServicesState> for Arc<TeamService<TeamRepository>> {
    fn from_ref(input: &ServicesState) -> Self {
        Arc::clone(&input.team_service)
    }
}

impl ServicesState {
    pub fn new(
        user_repository: UserRepository,
        country_repository: CountryRepository,
        marketplace_repository: MarketplaceRepository,
        product_repository: ProductRepository,
        team_repository: TeamRepository,
        email_client: ZeptoMail,
    ) -> Self {
        let user_service = Arc::new(UserService::new(user_repository.clone()));
        let auth_service = Arc::new(AuthenticationService::new(user_repository, email_client));
        let _admin_service =
            AdminService::new(Arc::clone(&user_service), Arc::clone(&auth_service));
        let country_service = Arc::new(CountryService::new(country_repository));
        let marketplace_service = Arc::new(MarketplaceService::new(marketplace_repository));
        let product_service = Arc::new(ProductService::new(product_repository));
        let team_service = Arc::new(TeamService::new(team_repository));
        let root_service = Arc::new(RootService::init());

        Self {
            user_service,
            auth_service,
            team_service,
            country_service,
            product_service,
            root_service,
            marketplace_service,
        }
    }
}

#[derive(Clone)]
pub struct GraphQlState {
    pub schema: Schema,
    pub endpoint: String,
}

#[derive(Clone)]
pub struct AppState {
    pub services: ServicesState,
    pub database_connection: DatabaseConnection,
    pub app_config: AppConfig,
}

impl AppState {
    pub fn new(db_conn: &DatabaseConnection) -> Result<Self, AppError> {
        let app_config = AppConfig::from_env()?;

        // repositories
        let country_repository = CountryRepository::init(db_conn);
        let _email_repository = EmailRepository::init(db_conn);
        let marketplace_repository = MarketplaceRepository::init(db_conn);
        let team_repository = TeamRepository::init(db_conn);
        let _upload_repository = UploadRepository::init(db_conn);
        let user_repository = UserRepository::init(db_conn);
        let product_repository = ProductRepository::init(db_conn);

        // externals
        let email_client = ZeptoMail::new(app_config.email_api_key.clone());

        // services
        let services = ServicesState::new(
            user_repository,
            country_repository,
            marketplace_repository,
            product_repository,
            team_repository,
            email_client,
        );

        Ok(Self {
            services,
            database_connection: db_conn.clone(),
            app_config,
        })
    }
}
