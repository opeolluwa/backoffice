use async_graphql::dynamic::Schema;
use axum::extract::FromRef;
use sea_orm::DatabaseConnection;
use seaography::async_graphql;

use backoffice_config::env::AppConfig;
use backoffice_domain::{
    errors::app_error::AppError,
    services::{
        auth::AuthenticationService, country::CountryService, emails::EmailsService,
        invitation::InvitationService, marketplace::MarketplaceService, product::ProductService,
        root::RootService, team::TeamService, upload::UploadsService, user::UserService,
    },
};
use backoffice_infra::{
    database::repositories::{
        base::Repository, country_repository::CountryRepository, email_repository::EmailRepository,
        invitation_repository::InvitationRepository, marketplace_repository::MarketplaceRepository,
        product_repository::ProductRepository, team_repository::TeamRepository,
        upload_repository::UploadRepository, user_repository::UserRepository,
    },
    imagekit::ImagekitClient,
    jwt::JwtTokenService,
    mailer::zepto_mailer::ZeptoMail,
};

#[derive(Clone)]
pub struct ServicesState {
    pub user_service: UserService<UserRepository>,
    pub root_service: RootService,
    pub auth_service: AuthenticationService<UserRepository, JwtTokenService, ZeptoMail>,
    pub marketplace_service: MarketplaceService<MarketplaceRepository>,
    pub product_service: ProductService<ProductRepository>,
    pub country_service: CountryService<CountryRepository>,
    pub team_service: TeamService<TeamRepository>,
    pub emails_service: EmailsService<EmailRepository>,
    pub invitation_service: InvitationService<InvitationRepository>,
    pub upload_service: UploadsService<UploadRepository, ImagekitClient>,
}

impl FromRef<AppState> for ServicesState {
    fn from_ref(state: &AppState) -> Self {
        state.services.clone()
    }
}

impl ServicesState {
    pub fn new(
        user_repository: UserRepository,
        country_repository: CountryRepository,
        marketplace_repository: MarketplaceRepository,
        product_repository: ProductRepository,
        team_repository: TeamRepository,
        email_repository: EmailRepository,
        invitation_repository: InvitationRepository,
        upload_repository: UploadRepository,
        email_client: ZeptoMail,
        imagekit_client: ImagekitClient,
    ) -> Self {
        let token_service = JwtTokenService::new();
        let user_service = UserService::new(user_repository.clone());
        let auth_service = AuthenticationService::new(user_repository, token_service, email_client);
        let country_service = CountryService::new(country_repository);
        let marketplace_service = MarketplaceService::new(marketplace_repository);
        let product_service = ProductService::new(product_repository);
        let team_service = TeamService::new(team_repository);
        let root_service = RootService::init();
        let emails_service = EmailsService::new(email_repository);
        let invitation_service = InvitationService::new(invitation_repository);
        let upload_service = UploadsService::new(upload_repository, imagekit_client);

        Self {
            user_service,
            auth_service,
            team_service,
            country_service,
            product_service,
            root_service,
            marketplace_service,
            emails_service,
            invitation_service,
            upload_service,
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
        let email_repository = EmailRepository::init(db_conn);
        let marketplace_repository = MarketplaceRepository::init(db_conn);
        let team_repository = TeamRepository::init(db_conn);
        let upload_repository = UploadRepository::init(db_conn);
        let user_repository = UserRepository::init(db_conn);
        let product_repository = ProductRepository::init(db_conn);
        let invitation_repository = InvitationRepository::init(db_conn);

        // externals
        let email_client = ZeptoMail::new(app_config.email_api_key.clone());

        let imagekit_client = ImagekitClient::new(
            &app_config.imagekit_public_key,
            &app_config.imagekit_private_key,
        )
        .map_err(|e| AppError::OperationFailed(e.to_string()))?;

        // services
        let services = ServicesState::new(
            user_repository,
            country_repository,
            marketplace_repository,
            product_repository,
            team_repository,
            email_repository,
            invitation_repository,
            upload_repository,
            email_client,
            imagekit_client,
        );

        Ok(Self {
            services,
            database_connection: db_conn.clone(),
            app_config,
        })
    }
}
