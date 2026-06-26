use sea_orm::DatabaseConnection;

use backoffice_email_client::zepto_mailer::ZeptoMail;

use crate::adapters::dto::user::UserDto;
use crate::config::app_config::AppConfig;
use crate::errors::service_error::ServiceError;
use crate::repositories::base::Repository;
use crate::repositories::user_repository::{UserRepository, UserRepositoryTrait};

#[derive(Clone)]
pub struct UserService {
    user_repository: UserRepository,
    email_client: ZeptoMail,
}

impl UserService {
    pub fn init(db: &DatabaseConnection, app_config: &AppConfig) -> Self {
        Self {
            user_repository: UserRepository::init(db),
            email_client: ZeptoMail::new(&app_config.email_api_key),
        }
    }
}

pub(crate) trait UserServiceTrait {
    async fn retrieve_information(&self, user_identifier: &str) -> Result<UserDto, ServiceError>;

    #[allow(dead_code)]
    async fn find_user_by_email(&self, user_email: &str) -> Result<UserDto, ServiceError>;
}

impl UserServiceTrait for UserService {
    async fn retrieve_information(&self, user_identifier: &str) -> Result<UserDto, ServiceError> {
        self.user_repository
            .retrieve_information(user_identifier)
            .await
    }
    async fn find_user_by_email(&self, user_email: &str) -> Result<UserDto, ServiceError> {
        self.user_repository
            .find_by_email(user_email)
            .await
            .ok_or(ServiceError::OperationFailed("user not found".to_string()))
            .map(|user| UserDto {
                identifier: user.identifier,
                email: user.email,
                first_name: user.first_name.unwrap_or_default(), //TODO: improve
                last_name: user.last_name.unwrap_or_default(),   //TODO: improve,
            })
    }
}
