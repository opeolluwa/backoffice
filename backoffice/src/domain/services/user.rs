use crate::api::http::extractors::dto::user::UserDto;
use crate::domain::ports::user_repository::UserRepositoryTrait;
use crate::errors::service_error::ServiceError;

#[derive(Clone)]
pub struct UserService<R: UserRepositoryTrait> {
    repo: R,
}

impl<R: UserRepositoryTrait + Clone> UserService<R> {
    pub fn new(repo: R) -> Self {
        Self { repo }
    }
}

pub(crate) trait UserServiceTrait {
    async fn retrieve_information(&self, user_identifier: &str) -> Result<UserDto, ServiceError>;

    #[allow(dead_code)]
    async fn find_user_by_email(&self, user_email: &str) -> Result<UserDto, ServiceError>;
}

impl<R: UserRepositoryTrait + Clone + Send + Sync> UserServiceTrait for UserService<R> {
    async fn retrieve_information(&self, user_identifier: &str) -> Result<UserDto, ServiceError> {
        self.repo.retrieve_information(user_identifier).await
    }

    async fn find_user_by_email(&self, user_email: &str) -> Result<UserDto, ServiceError> {
        self.repo
            .find_by_email(user_email)
            .await
            .ok_or(ServiceError::OperationFailed("user not found".to_string()))
            .map(|user| UserDto {
                identifier: user.identifier,
                email: user.email,
                first_name: user.first_name.unwrap_or_default(),
                last_name: user.last_name.unwrap_or_default(),
            })
    }
}
