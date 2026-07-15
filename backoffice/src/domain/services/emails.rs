use crate::{
    domain::{
        dto::{CreateEmailCommand, UpdateEmailCommand},
        models::emails,
        ports::email_repository::EmailRepositoryExt,
    },
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
        command: &CreateEmailCommand,
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
        command: &UpdateEmailCommand,
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
        command: &CreateEmailCommand,
        user_identifier: &str,
    ) -> Result<emails::Model, ServiceError> {
        Ok(self.repo.create_email(command, user_identifier).await?)
    }

    async fn find_email_by_identifier(
        &self,
        identifier: &str,
        user_identifier: &str,
    ) -> Result<emails::Model, ServiceError> {
        Ok(self
            .repo
            .find_email_by_identifier(identifier, user_identifier)
            .await?)
    }

    async fn find_all_emails(
        &self,
        user_identifier: &str,
    ) -> Result<Vec<emails::Model>, ServiceError> {
        Ok(self.repo.find_all_emails(user_identifier).await?)
    }

    async fn find_emails_by_tag(
        &self,
        tag: &str,
        user_identifier: &str,
    ) -> Result<Vec<emails::Model>, ServiceError> {
        Ok(self.repo.find_emails_by_tag(tag, user_identifier).await?)
    }

    async fn find_starred_emails(
        &self,
        user_identifier: &str,
    ) -> Result<Vec<emails::Model>, ServiceError> {
        Ok(self.repo.find_starred_emails(user_identifier).await?)
    }

    async fn find_unread_emails(
        &self,
        user_identifier: &str,
    ) -> Result<Vec<emails::Model>, ServiceError> {
        Ok(self.repo.find_unread_emails(user_identifier).await?)
    }

    async fn update_email(
        &self,
        identifier: &str,
        command: &UpdateEmailCommand,
        user_identifier: &str,
    ) -> Result<emails::Model, ServiceError> {
        Ok(self
            .repo
            .update_email(identifier, command, user_identifier)
            .await?)
    }

    async fn delete_email(
        &self,
        identifier: &str,
        user_identifier: &str,
    ) -> Result<(), ServiceError> {
        Ok(self
            .repo
            .delete_email(identifier, user_identifier)
            .await?)
    }

    async fn count_emails(&self, user_identifier: &str) -> Result<i64, ServiceError> {
        Ok(self.repo.count_emails(user_identifier).await?)
    }

    async fn count_unread_emails(&self, user_identifier: &str) -> Result<i64, ServiceError> {
        Ok(self.repo.count_unread_emails(user_identifier).await?)
    }
}
