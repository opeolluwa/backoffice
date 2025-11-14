use std::sync::Arc;

use crate::{
    adapters::{dto::user::UserDto, requests::auth::CreateUserRequest},
    entities::user::User,
    errors::{common_service_error::ServiceError, user_service_error::UserServiceError}, repositories::base::Repository,
};
use sqlx::{Pool, Postgres};
use ulid::Ulid;

#[derive(Clone)]
pub struct UserRepository {
    pool: Arc<Pool<Postgres>>,
}

impl Repository for UserRepository {
    fn init(pool: &Pool<Postgres>) -> Self {
        Self {
            pool: Arc::new(pool.clone()),
        }
    }
}


pub trait UserRepositoryTrait {
    fn find_by_identifier(
        &self,
        identifier: &str,
    ) -> impl std::future::Future<Output = Option<User>> + Send;

    fn find_by_email(&self, email: &str) -> impl std::future::Future<Output = Option<User>> + Send;

    fn update_account_status(
        &self,
        identifier: &str,
    ) -> impl std::future::Future<Output = Result<(), ServiceError>> + Send;

    fn update_password(
        &self,
        identifier: &str,
        new_password: &str,
    ) -> impl std::future::Future<Output = Result<(), ServiceError>> + Send;

    fn create_user(
        &self,
        user: CreateUserRequest,
    ) -> impl std::future::Future<Output = Result<(), UserServiceError>> + Send;

    fn retrieve_information(
        &self,
        identifier: &str,
    ) -> impl std::future::Future<Output = Result<UserDto, UserServiceError>> + Send;
}

impl UserRepositoryTrait for UserRepository {
    async fn find_by_identifier(&self, identifier: &str) -> Option<User> {
        sqlx::query_as::<_, User>("SELECT * FROM users WHERE identifier = $1")
            .bind(identifier)
            .fetch_one(self.pool.as_ref())
            .await
            .ok()
    }
    async fn find_by_email(&self, email: &str) -> Option<User> {
        sqlx::query_as::<_, User>("SELECT * FROM users WHERE email = $1")
            .bind(email)
            .fetch_one(self.pool.as_ref())
            .await
            .ok()
    }

    async fn update_account_status(&self, identifier: &str) -> Result<(), ServiceError> {
        let _ = sqlx::query_as::<_, User>("UPDATE users SET is_active = $1  WHERE identifier = $2")
            .bind(true)
            .bind(identifier.to_string())
            .fetch_one(self.pool.as_ref())
            .await
            .map_err(|err| UserServiceError::OperationFailed(err.to_string()));

        Ok(())
    }

    async fn update_password(
        &self,
        identifier: &str,
        new_password: &str,
    ) -> Result<(), ServiceError> {
        let _ = sqlx::query_as::<_, User>("UPDATE users SET password = $1  WHERE identifier  = $2")
            .bind(new_password)
            .bind(identifier)
            .fetch_one(self.pool.as_ref())
            .await
            .map_err(|err| UserServiceError::OperationFailed(err.to_string()));

        Ok(())
    }

    async fn create_user(&self, user: CreateUserRequest) -> Result<(), UserServiceError> {
        sqlx::query(
            "INSERT INTO users (identifier, first_name, last_name, email, password) VALUES ($1, $2, $3, $4, $5)"
        )
            .bind(Ulid::new().to_string())
            .bind(user.first_name)
            .bind(user.last_name)
            .bind(user.email)
            .bind(user.password)
            .execute(self.pool.as_ref())
            .await
            .map_err(|err| UserServiceError::OperationFailed(err.to_string()))?;

        Ok(())
    }
    async fn retrieve_information(&self, identifier: &str) -> Result<UserDto, UserServiceError> {
        sqlx::query_as::<_, UserDto>(r#"SELECT * FROM users  WHERE identifier = $1"#)
            .bind(identifier)
            .fetch_one(self.pool.as_ref())
            .await
            .map_err(UserServiceError::from)
    }
}
