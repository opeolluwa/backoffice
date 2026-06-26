use crate::{
    api::http::extractors::{dto::user::UserDto, requests::auth::CreateUserRequest},
    entities::users,
    errors::service_error::ServiceError,
};

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
