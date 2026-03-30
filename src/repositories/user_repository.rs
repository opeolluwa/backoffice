use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set};
use ulid::Ulid;

use crate::{
    adapters::{dto::user::UserDto, requests::auth::CreateUserRequest},
    entities::users::{self, Entity as UserEntity},
    errors::service_error::ServiceError,
    repositories::base::Repository,
};

#[derive(Clone)]
pub struct UserRepository {
    db: DatabaseConnection,
}

impl Repository for UserRepository {
    fn init(db: &DatabaseConnection) -> Self {
        Self { db: db.clone() }
    }
}

pub trait UserRepositoryTrait {
    fn find_by_identifier(
        &self,
        identifier: &str,
    ) -> impl std::future::Future<Output = Option<users::Model>> + Send;

    fn find_by_email(
        &self,
        email: &str,
    ) -> impl std::future::Future<Output = Option<users::Model>> + Send;

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
    ) -> impl std::future::Future<Output = Result<(), ServiceError>> + Send;

    fn retrieve_information(
        &self,
        identifier: &str,
    ) -> impl std::future::Future<Output = Result<UserDto, ServiceError>> + Send;
}

impl UserRepositoryTrait for UserRepository {
    async fn find_by_identifier(&self, identifier: &str) -> Option<users::Model> {
        UserEntity::find_by_id(identifier)
            .one(&self.db)
            .await
            .ok()
            .flatten()
    }

    async fn find_by_email(&self, email: &str) -> Option<users::Model> {
        UserEntity::find()
            .filter(users::Column::Email.eq(email))
            .one(&self.db)
            .await
            .ok()
            .flatten()
    }

    async fn update_account_status(&self, identifier: &str) -> Result<(), ServiceError> {
        let model = users::ActiveModel {
            identifier: Set(identifier.to_string()),
            is_active: Set(true),
            ..Default::default()
        };
        UserEntity::update(model)
            .exec(&self.db)
            .await
            .map_err(|err| ServiceError::OperationFailed(err.to_string()))?;
        Ok(())
    }

    async fn update_password(
        &self,
        identifier: &str,
        new_password: &str,
    ) -> Result<(), ServiceError> {
        let model = users::ActiveModel {
            identifier: Set(identifier.to_string()),
            password: Set(new_password.to_string()),
            ..Default::default()
        };
        UserEntity::update(model)
            .exec(&self.db)
            .await
            .map_err(|err| ServiceError::OperationFailed(err.to_string()))?;
        Ok(())
    }

    async fn create_user(&self, user: CreateUserRequest) -> Result<(), ServiceError> {
        let model = users::ActiveModel {
            identifier: Set(Ulid::new().to_string()),
            first_name: Set(Some(user.first_name)),
            last_name: Set(Some(user.last_name)),
            email: Set(user.email),
            password: Set(user.password),
            ..Default::default()
        };
        model
            .insert(&self.db)
            .await
            .map_err(|err| ServiceError::OperationFailed(err.to_string()))?;
        Ok(())
    }

    async fn retrieve_information(&self, identifier: &str) -> Result<UserDto, ServiceError> {
        let user = UserEntity::find_by_id(identifier)
            .one(&self.db)
            .await
            .map_err(|err| ServiceError::OperationFailed(err.to_string()))?
            .ok_or_else(|| ServiceError::OperationFailed("user not found".to_string()))?;

        Ok(UserDto {
            identifier: user.identifier,
            email: user.email,
            first_name: user.first_name.unwrap_or_default(),
            last_name: user.last_name.unwrap_or_default(),
        })
    }
}
