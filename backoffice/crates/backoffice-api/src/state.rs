use async_graphql::dynamic::Schema;
use axum::extract::FromRef;
use backoffice_payment_provider::paystack::PaystackClient;
use sea_orm::DatabaseConnection;
use seaography::async_graphql;

use backoffice_config::env::AppConfig;
use backoffice_domain::{
    errors::app_error::AppError,
    services::{
        auth::AuthenticationService, country::CountryService, emails::EmailsService,
        invitation::InvitationService, marketplace::MarketplaceService,
        newsletter::NewsletterService, product::ProductService, root::RootService,
        team::TeamService, upload::UploadsService, user::UserService,
    },
};
use backoffice_infra::{
    database::repositories::{
        app_config_repository::AppConfigRepository, base::Repository,
        country_repository::CountryRepository, email_repository::EmailRepository,
        invitation_repository::InvitationRepository, marketplace_repository::MarketplaceRepository,
        newsletter_repository::NewsletterRepository, product_repository::ProductRepository,
        role_repository::RoleRepository, team_repository::TeamRepository,
        upload_repository::UploadRepository, user_repository::UserRepository,
    },
    imagekit::ImagekitClient,
    jwt::JwtTokenService,
    mailer::smtp::SmtpEmailSender,
};

#[derive(Clone)]
pub struct Repositories {
    pub user: UserRepository,
    pub team: TeamRepository,
    pub country: CountryRepository,
    pub marketplace: MarketplaceRepository,
    pub product: ProductRepository,
    pub email: EmailRepository,
    pub invitation: InvitationRepository,
    pub upload: UploadRepository,
    pub newsletter: NewsletterRepository,
    pub role: RoleRepository,
    pub app_config: AppConfigRepository,
}

#[derive(Clone)]
pub struct Contracts {
    pub email: SmtpEmailSender,
    pub imagekit: ImagekitClient,
    pub paystack: PaystackClient,
}

#[derive(Clone)]
pub struct ServicesState {
    pub user_service: UserService<UserRepository>,
    pub root_service: RootService,
    pub auth_service: AuthenticationService<UserRepository, JwtTokenService, SmtpEmailSender>,
    pub marketplace_service: MarketplaceService<MarketplaceRepository>,
    pub product_service: ProductService<ProductRepository>,
    pub country_service: CountryService<CountryRepository>,
    pub team_service: TeamService<TeamRepository>,
    pub emails_service: EmailsService<EmailRepository>,
    pub invitation_service: InvitationService<InvitationRepository>,
    pub upload_service: UploadsService<UploadRepository, ImagekitClient>,
    pub paystack_client: PaystackClient,
    pub newsletter_service: NewsletterService<NewsletterRepository>,
}

#[derive(Clone)]
pub struct GraphQlState {
    pub schema: Schema,
    pub endpoint: String,
}

pub struct Application {
    pub config: AppConfig,
    pub db: DatabaseConnection,
    pub repositories: Repositories,
    pub services: ServicesState,
}

#[derive(Clone)]
pub struct AppState {
    pub services: ServicesState,
    pub database_connection: DatabaseConnection,
    pub app_config: AppConfig,
}

impl Repositories {
    pub fn new(db: &DatabaseConnection) -> Self {
        Self {
            user: UserRepository::init(db),
            team: TeamRepository::init(db),
            country: CountryRepository::init(db),
            marketplace: MarketplaceRepository::init(db),
            product: ProductRepository::init(db),
            email: EmailRepository::init(db),
            invitation: InvitationRepository::init(db),
            upload: UploadRepository::init(db),
            newsletter: NewsletterRepository::init(db),
            role: RoleRepository::init(db),
            app_config: AppConfigRepository::init(db),
        }
    }
}

impl Contracts {
    pub fn new(app_config: &AppConfig) -> Result<Self, AppError> {
        let email = SmtpEmailSender::new(
            &app_config.smtp_host,
            app_config.smtp_port,
            &app_config.smtp_username,
            &app_config.smtp_password,
        )
        .map_err(|e| AppError::OperationFailed(e.to_string()))?;

        let paystack = PaystackClient::new(
            &app_config.paystack_api_secret,
            &app_config.paystack_base_url,
        );

        let imagekit = ImagekitClient::new(
            &app_config.imagekit_public_key,
            &app_config.imagekit_private_key,
        )
        .map_err(|e| AppError::OperationFailed(e.to_string()))?;

        Ok(Self {
            email,
            imagekit,
            paystack,
        })
    }
}

impl ServicesState {
    pub fn new(repos: Repositories, contracts: Contracts) -> Self {
        let token_service = JwtTokenService::new();

        Self {
            user_service: UserService::new(repos.user.clone()),
            auth_service: AuthenticationService::new(
                repos.user,
                token_service,
                contracts.email.clone(),
            ),
            country_service: CountryService::new(repos.country),
            marketplace_service: MarketplaceService::new(repos.marketplace),
            product_service: ProductService::new(repos.product),
            team_service: TeamService::new(repos.team),
            emails_service: EmailsService::new(repos.email),
            invitation_service: InvitationService::new(repos.invitation),
            upload_service: UploadsService::new(repos.upload, contracts.imagekit.clone()),
            newsletter_service: NewsletterService::new(repos.newsletter),
            root_service: RootService::init(),
            paystack_client: contracts.paystack,
        }
    }
}

impl FromRef<AppState> for ServicesState {
    fn from_ref(state: &AppState) -> Self {
        state.services.clone()
    }
}

impl Application {
    pub fn new(db: &DatabaseConnection) -> Result<Self, AppError> {
        let config = AppConfig::from_env()?;
        let contracts = Contracts::new(&config)?;
        let repositories = Repositories::new(db);

        let services = ServicesState::new(repositories.clone(), contracts);

        Ok(Self {
            services,
            db: db.clone(),
            config,
            repositories,
        })
    }
}

impl AppState {
    pub fn new(db: &DatabaseConnection) -> Result<Self, AppError> {
        let app_config = AppConfig::from_env()?;
        let contracts = Contracts::new(&app_config)?;
        let repositories = Repositories::new(db);

        let services = ServicesState::new(repositories, contracts);

        Ok(Self {
            services,
            database_connection: db.clone(),
            app_config,
        })
    }
}