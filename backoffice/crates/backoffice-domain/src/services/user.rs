use crate::errors::service_error::ServiceError;
use crate::{dto::UserProfile, ports::user_repository::UserRepositoryTrait};

#[derive(Clone)]
pub struct UserService<R: UserRepositoryTrait+ Send + Sync> {
    repo: R,
}

impl<R: UserRepositoryTrait+ Send + Sync> UserService<R> {
    pub fn new(repo: R) -> Self {
        Self { repo }
    }
}

pub trait UserServiceTrait {
    async fn retrieve_information(
        &self,
        user_identifier: &str,
    ) -> Result<UserProfile, ServiceError>;

    async fn find_user_by_email(&self, user_email: &str) -> Result<UserProfile, ServiceError>;
}

impl<R: UserRepositoryTrait + Send + Sync> UserServiceTrait for UserService<R> {
    async fn retrieve_information(
        &self,
        user_identifier: &str,
    ) -> Result<UserProfile, ServiceError> {
        self.repo.retrieve_information(user_identifier).await
    }

    async fn find_user_by_email(&self, user_email: &str) -> Result<UserProfile, ServiceError> {
        let user = self
            .repo
            .find_by_email(user_email)
            .await
            .ok_or(ServiceError::OperationFailed("user not found".to_string()))?;

        Ok(UserProfile {
            identifier: user.identifier,
            email: user.email,
            first_name: user.first_name.unwrap_or_default(),
            last_name: user.last_name.unwrap_or_default(),
        })
    }
}
