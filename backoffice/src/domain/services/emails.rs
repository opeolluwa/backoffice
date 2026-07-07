use crate::{
    api::http::extractors::email::{CreateEmailRequest, UpdateEmailRequest},
    domain::models::emails,
    domain::ports::email_repository::EmailRepositoryExt,
    errors::service_error::ServiceError,
};

pub struct EmailsService<R: EmailRepositoryExt> {
    repo: R,
}

impl<R: EmailRepositoryExt> EmailsService<R> {
    pub fn new(repo: R) -> Self {
        Self { repo }
    }
}

pub(crate) trait EmailsServiceExt {
    async fn create_email(
        &self,
        request: &CreateEmailRequest,
        user_identifier: &str,
    ) -> Result<emails::Model, ServiceError>;

    async fn find_email_by_identifier(
        &self,
        identifier: &str,
        user_identifier: &str,
    ) -> Result<emails::Model, ServiceError>;

    async fn find_all_emails(
        &self,
        user_identifier: &str,
    ) -> Result<Vec<emails::Model>, ServiceError>;

    async fn find_emails_by_tag(
        &self,
        tag: &str,
        user_identifier: &str,
    ) -> Result<Vec<emails::Model>, ServiceError>;

    async fn find_starred_emails(
        &self,
        user_identifier: &str,
    ) -> Result<Vec<emails::Model>, ServiceError>;

    async fn find_unread_emails(
        &self,
        user_identifier: &str,
    ) -> Result<Vec<emails::Model>, ServiceError>;

    async fn update_email(
        &self,
        identifier: &str,
        request: &UpdateEmailRequest,
        user_identifier: &str,
    ) -> Result<emails::Model, ServiceError>;

    async fn delete_email(
        &self,
        identifier: &str,
        user_identifier: &str,
    ) -> Result<(), ServiceError>;

    async fn count_emails(&self, user_identifier: &str) -> Result<i64, ServiceError>;

    async fn count_unread_emails(&self, user_identifier: &str) -> Result<i64, ServiceError>;
}

impl<R: EmailRepositoryExt + Send + Sync> EmailsServiceExt for EmailsService<R> {
    async fn create_email(
        &self,
        request: &CreateEmailRequest,
        user_identifier: &str,
    ) -> Result<emails::Model, ServiceError> {
        self.repo
            .create_email(request, user_identifier)
            .await
            .map_err(|e| ServiceError::OperationFailed(e.to_string()))
    }

    async fn find_email_by_identifier(
        &self,
        identifier: &str,
        user_identifier: &str,
    ) -> Result<emails::Model, ServiceError> {
        self.repo
            .find_email_by_identifier(identifier, user_identifier)
            .await
            .map_err(|e| ServiceError::OperationFailed(e.to_string()))
    }

    async fn find_all_emails(
        &self,
        user_identifier: &str,
    ) -> Result<Vec<emails::Model>, ServiceError> {
        self.repo
            .find_all_emails(user_identifier)
            .await
            .map_err(|e| ServiceError::OperationFailed(e.to_string()))
    }

    async fn find_emails_by_tag(
        &self,
        tag: &str,
        user_identifier: &str,
    ) -> Result<Vec<emails::Model>, ServiceError> {
        self.repo
            .find_emails_by_tag(tag, user_identifier)
            .await
            .map_err(|e| ServiceError::OperationFailed(e.to_string()))
    }

    async fn find_starred_emails(
        &self,
        user_identifier: &str,
    ) -> Result<Vec<emails::Model>, ServiceError> {
        self.repo
            .find_starred_emails(user_identifier)
            .await
            .map_err(|e| ServiceError::OperationFailed(e.to_string()))
    }

    async fn find_unread_emails(
        &self,
        user_identifier: &str,
    ) -> Result<Vec<emails::Model>, ServiceError> {
        self.repo
            .find_unread_emails(user_identifier)
            .await
            .map_err(|e| ServiceError::OperationFailed(e.to_string()))
    }

    async fn update_email(
        &self,
        identifier: &str,
        request: &UpdateEmailRequest,
        user_identifier: &str,
    ) -> Result<emails::Model, ServiceError> {
        self.repo
            .update_email(identifier, request, user_identifier)
            .await
            .map_err(|e| ServiceError::OperationFailed(e.to_string()))
    }

    async fn delete_email(
        &self,
        identifier: &str,
        user_identifier: &str,
    ) -> Result<(), ServiceError> {
        self.repo
            .delete_email(identifier, user_identifier)
            .await
            .map_err(|e| ServiceError::OperationFailed(e.to_string()))
    }

    async fn count_emails(&self, user_identifier: &str) -> Result<i64, ServiceError> {
        self.repo
            .count_emails(user_identifier)
            .await
            .map_err(|e| ServiceError::OperationFailed(e.to_string()))
    }

    async fn count_unread_emails(&self, user_identifier: &str) -> Result<i64, ServiceError> {
        self.repo
            .count_unread_emails(user_identifier)
            .await
            .map_err(|e| ServiceError::OperationFailed(e.to_string()))
    }
}
