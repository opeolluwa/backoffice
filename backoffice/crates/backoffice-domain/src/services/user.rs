use std::future::Future;

use crate::errors::service_error::ServiceError;
use crate::{dto::UserProfile, ports::user_repository::UserRepositoryTrait};

#[derive(Clone)]
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ports::user_repository::MockUserRepositoryTrait;
    use sea_orm::sqlx::types::chrono::Utc;

    fn test_user_model() -> crate::models::users::Model {
        crate::models::users::Model {
            identifier: "01HXYZ0000000000000000001".to_string(),
            email: "test@example.com".to_string(),
            password: "hashed".to_string(),
            first_name: Some("John".to_string()),
            last_name: Some("Doe".to_string()),
            created_at: Utc::now().naive_utc().and_utc().into(),
            updated_at: None,
            is_active: true,
            role_identifier: None,
        }
    }

    fn test_user_profile() -> UserProfile {
        UserProfile {
            identifier: "01HXYZ0000000000000000001".to_string(),
            email: "test@example.com".to_string(),
            first_name: "John".to_string(),
            last_name: "Doe".to_string(),
        }
    }

    #[tokio::test]
    async fn retrieve_information_delegates_to_repo() {
        let mut repo = MockUserRepositoryTrait::new();
        let profile = test_user_profile();
        repo.expect_retrieve_information()
            .returning(move |_| {
                let p = profile.clone();
                Box::pin(async move { Ok(p) })
            });
        let service = UserService::new(repo);

        let result = service.retrieve_information("user-123").await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap().email, "test@example.com");
    }

    #[tokio::test]
    async fn find_user_by_email_found() {
        let mut repo = MockUserRepositoryTrait::new();
        let user = test_user_model();
        repo.expect_find_by_email()
            .returning(move |_| {
                let u = user.clone();
                Box::pin(async move { Some(u) })
            });
        let service = UserService::new(repo);

        let result = service.find_user_by_email("test@example.com").await;
        assert!(result.is_ok());
        let profile = result.unwrap();
        assert_eq!(profile.first_name, "John");
        assert_eq!(profile.last_name, "Doe");
    }

    #[tokio::test]
    async fn find_user_by_email_not_found() {
        let mut repo = MockUserRepositoryTrait::new();
        repo.expect_find_by_email()
            .returning(|_| Box::pin(async { None }));
        let service = UserService::new(repo);

        let result = service.find_user_by_email("nobody@example.com").await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn find_user_by_email_empty_names_defaults() {
        let mut repo = MockUserRepositoryTrait::new();
        let mut user = test_user_model();
        user.first_name = None;
        user.last_name = None;
        repo.expect_find_by_email()
            .returning(move |_| {
                let u = user.clone();
                Box::pin(async move { Some(u) })
            });
        let service = UserService::new(repo);

        let result = service.find_user_by_email("test@example.com").await;
        let profile = result.unwrap();
        assert_eq!(profile.first_name, "");
        assert_eq!(profile.last_name, "");
    }
}
