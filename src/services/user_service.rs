use sqlx::{Pool, Postgres};


use crate::adapters::dto::user::UserDto;
use crate::errors::user_service_error::UserServiceError;
use crate::repositories::user_repository::{UserRepository, UserRepositoryTrait};


#[derive(Clone)]
pub struct UserService {
    user_repository: UserRepository,
}

impl UserService {
    pub fn init(pool: &Pool<Postgres>) -> Self {
        Self {
            user_repository: UserRepository::init(pool),
        }
    }
}

pub(crate) trait UserServiceTrait {
    async fn retrieve_information(
        &self,
        user_identifier: &str,
    ) -> Result<UserDto, UserServiceError>;

    async fn find_user_by_email(
        &self,
        user_email: &str,
    ) -> Result<UserDto, UserServiceError>;
    
    
}

impl UserServiceTrait for UserService {
    async fn retrieve_information(
        &self,
        user_identifier: &str,
    ) -> Result<UserDto, UserServiceError> {
        self.user_repository
            .retrieve_information(&user_identifier)
            .await
    }
    async fn find_user_by_email(&self, user_email: &str) -> Result<UserDto, UserServiceError> {
        self.user_repository.find_by_email(user_email).await.ok_or(UserServiceError::OperationFailed("user not found".to_string())).map(|user| UserDto {
            identifier: user.identifier,
            email: user.email,
            first_name: user.first_name,
            last_name: user.last_name,
        })
    }
}
