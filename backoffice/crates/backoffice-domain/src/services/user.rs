use crate::{
    dto::{ChangePasswordCommand, UpdateProfileCommand, UserProfile},
    errors::service_error::ServiceError,
    ports::user_repository::UserRepositoryTrait,
    services::user_helper::{UserHelperService, UserHelperServiceTrait},
};

#[derive(Clone)]
pub struct UserService<R: UserRepositoryTrait + Send + Sync> {
    repo: R,
    user_helper_service: UserHelperService,
}

impl<R: UserRepositoryTrait + Send + Sync> UserService<R> {
    pub fn new(repo: R) -> Self {
        Self {
            repo,
            user_helper_service: UserHelperService::init(),
        }
    }
}

pub trait UserServiceTrait {
    async fn retrieve_information(
        &self,
        user_identifier: &str,
    ) -> Result<UserProfile, ServiceError>;

    async fn find_user_by_email(&self, user_email: &str) -> Result<UserProfile, ServiceError>;

    async fn change_password(
        &self,
        identifier: &str,
        command: &ChangePasswordCommand,
    ) -> Result<(), ServiceError>;

    async fn update_profile(
        &self,
        identifier: &str,
        command: &UpdateProfileCommand,
    ) -> Result<(), ServiceError>;

    async fn update_profile_picture(
        &self,
        identifier: &str,
        url: &str,
    ) -> Result<(), ServiceError>;
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
            profile_picture: user.profile_picture,
            username: user.username,
        })
    }

    async fn change_password(
        &self,
        identifier: &str,
        command: &ChangePasswordCommand,
    ) -> Result<(), ServiceError> {
        if command.new_password != command.confirm_password {
            return Err(ServiceError::OperationFailed(
                "passwords do not match".to_string(),
            ));
        }

        let user = self
            .repo
            .find_by_identifier(identifier)
            .await
            .ok_or_else(|| ServiceError::OperationFailed("user not found".to_string()))?;

        let valid = self
            .user_helper_service
            .validate_password(&command.current_password, &user.password)?;

        if !valid {
            return Err(ServiceError::OperationFailed(
                "current password is incorrect".to_string(),
            ));
        }

        let hashed = self.user_helper_service.hash_password(&command.new_password)?;
        self.repo.update_password(identifier, &hashed).await
    }

    async fn update_profile(
        &self,
        identifier: &str,
        command: &UpdateProfileCommand,
    ) -> Result<(), ServiceError> {
        self.repo.update_profile(identifier, command).await
    }

    async fn update_profile_picture(
        &self,
        identifier: &str,
        url: &str,
    ) -> Result<(), ServiceError> {
        self.repo.update_profile_picture(identifier, url).await
    }
}
