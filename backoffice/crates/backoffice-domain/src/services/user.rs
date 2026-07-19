use std::future::Future;

use crate::errors::service_error::ServiceError;
use crate::{dto::UserProfile, ports::user_repository::UserRepositoryTrait};

pub struct UserService<R: UserRepositoryTrait> {
    repo: R,
}

impl<R: UserRepositoryTrait> UserService<R> {
    pub fn new(repo: R) -> Self {
        Self { repo }
    }
}

pub trait UserServiceTrait {
    fn retrieve_information(
        &self,
        user_identifier: &str,
    ) -> impl Future<Output = Result<UserProfile, ServiceError>> + Send;

    #[allow(dead_code)]
    fn find_user_by_email(
        &self,
        user_email: &str,
    ) -> impl Future<Output = Result<UserProfile, ServiceError>> + Send;
}

impl<R: UserRepositoryTrait + Send + Sync> UserServiceTrait for UserService<R> {
    fn retrieve_information(
        &self,
        user_identifier: &str,
    ) -> impl Future<Output = Result<UserProfile, ServiceError>> + Send {
        let user_identifier = user_identifier.to_owned();
        let repo = &self.repo;
        async move { repo.retrieve_information(&user_identifier).await }
    }

    fn find_user_by_email(
        &self,
        user_email: &str,
    ) -> impl Future<Output = Result<UserProfile, ServiceError>> + Send {
        let user_email = user_email.to_owned();
        let repo = &self.repo;
        async move {
            let user = repo
                .find_by_email(&user_email)
                .await
                .ok_or_else(|| ServiceError::OperationFailed("user not found".to_string()))?;

            Ok(UserProfile {
                identifier: user.identifier,
                email: user.email,
                first_name: user.first_name.unwrap_or_default(),
                last_name: user.last_name.unwrap_or_default(),
            })
        }
    }
}
