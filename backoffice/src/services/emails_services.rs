use crate::{
    dto::{CreateEmailCommand, UpdateEmailCommand},
    errors::service_error::ServiceError,
    models::emails,
    repositories::email_repository::EmailRepositoryExt,
};

#[derive(Clone)]
pub struct EmailsService<R: EmailRepositoryExt> {
    repo: R,
}

impl<R: EmailRepositoryExt> EmailsService<R> {
    pub fn new(repo: R) -> Self {
        Self { repo }
    }
}

pub trait EmailsServiceExt {
    async fn create_email(
        &self,
        command: &CreateEmailCommand,
    ) -> Result<emails::Model, ServiceError>;

    async fn find_email_by_identifier(
        &self,
        identifier: &str,
    ) -> Result<emails::Model, ServiceError>;

    async fn find_all_emails(&self) -> Result<Vec<emails::Model>, ServiceError>;

    async fn find_emails_by_tag(&self, tag: &str) -> Result<Vec<emails::Model>, ServiceError>;

    async fn find_starred_emails(&self) -> Result<Vec<emails::Model>, ServiceError>;

    async fn find_unread_emails(&self) -> Result<Vec<emails::Model>, ServiceError>;

    async fn update_email(
        &self,
        identifier: &str,
        command: &UpdateEmailCommand,
    ) -> Result<emails::Model, ServiceError>;

    async fn delete_email(&self, identifier: &str) -> Result<(), ServiceError>;

    async fn count_emails(&self) -> Result<i64, ServiceError>;

    async fn count_unread_emails(&self) -> Result<i64, ServiceError>;
}

impl<R: EmailRepositoryExt + Send + Sync> EmailsServiceExt for EmailsService<R> {
    async fn create_email(
        &self,
        command: &CreateEmailCommand,
    ) -> Result<emails::Model, ServiceError> {
        Ok(self.repo.create_email(command).await?)
    }

    async fn find_email_by_identifier(
        &self,
        identifier: &str,
    ) -> Result<emails::Model, ServiceError> {
        Ok(self.repo.find_email_by_identifier(identifier).await?)
    }

    async fn find_all_emails(&self) -> Result<Vec<emails::Model>, ServiceError> {
        Ok(self.repo.find_all_emails().await?)
    }

    async fn find_emails_by_tag(&self, tag: &str) -> Result<Vec<emails::Model>, ServiceError> {
        Ok(self.repo.find_emails_by_tag(tag).await?)
    }

    async fn find_starred_emails(&self) -> Result<Vec<emails::Model>, ServiceError> {
        Ok(self.repo.find_starred_emails().await?)
    }

    async fn find_unread_emails(&self) -> Result<Vec<emails::Model>, ServiceError> {
        Ok(self.repo.find_unread_emails().await?)
    }

    async fn update_email(
        &self,
        identifier: &str,
        command: &UpdateEmailCommand,
    ) -> Result<emails::Model, ServiceError> {
        Ok(self.repo.update_email(identifier, command).await?)
    }

    async fn delete_email(&self, identifier: &str) -> Result<(), ServiceError> {
        Ok(self.repo.delete_email(identifier).await?)
    }

    async fn count_emails(&self) -> Result<i64, ServiceError> {
        Ok(self.repo.count_emails().await?)
    }

    async fn count_unread_emails(&self) -> Result<i64, ServiceError> {
        Ok(self.repo.count_unread_emails().await?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::repositories::email_repository::MockEmailRepositoryExt;
    use sea_orm::sqlx::types::chrono::Utc;

    fn test_email() -> emails::Model {
        emails::Model {
            identifier: "em-001".to_string(),
            subject: "Test Subject".to_string(),
            body: "Hello world".to_string(),
            sender_email: "sender@example.com".to_string(),
            recipient_email: "recipient@example.com".to_string(),
            date_sent: Utc::now().naive_utc().and_utc().into(),
            tag: Some("work".to_string()),
            is_read: false,
            is_starred: false,
            has_attachments: false,
            data: None,
            user_identifier: Some("user-001".to_string()),
            created_at: Utc::now().naive_utc().and_utc().into(),
            updated_at: None,
        }
    }

    #[tokio::test]
    async fn create_email_returns_model() {
        let mut repo = MockEmailRepositoryExt::new();
        let email = test_email();
        repo.expect_create_email()
            .returning(move |_| Ok(email.clone()));
        let service = EmailsService::new(repo);

        let cmd = crate::dto::CreateEmailCommand {
            subject: "Test Subject".to_string(),
            body: "Hello world".to_string(),
            sender_email: "sender@example.com".to_string(),
            recipient_email: "recipient@example.com".to_string(),
            tag: Some("work".to_string()),
            has_attachments: None,
            data: None,
        };
        let result = service.create_email(&cmd).await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap().subject, "Test Subject");
    }

    #[tokio::test]
    async fn find_email_by_identifier() {
        let mut repo = MockEmailRepositoryExt::new();
        let email = test_email();
        repo.expect_find_email_by_identifier()
            .returning(move |_| Ok(email.clone()));
        let service = EmailsService::new(repo);

        assert!(service.find_email_by_identifier("em-001").await.is_ok());
    }

    #[tokio::test]
    async fn find_all_emails() {
        let mut repo = MockEmailRepositoryExt::new();
        repo.expect_find_all_emails()
            .returning(|| Ok(vec![test_email()]));
        let service = EmailsService::new(repo);

        assert_eq!(service.find_all_emails().await.unwrap().len(), 1);
    }

    #[tokio::test]
    async fn count_emails() {
        let mut repo = MockEmailRepositoryExt::new();
        repo.expect_count_emails().returning(|| Ok(10));
        let service = EmailsService::new(repo);

        assert_eq!(service.count_emails().await.unwrap(), 10);
    }

    #[tokio::test]
    async fn count_unread_emails() {
        let mut repo = MockEmailRepositoryExt::new();
        repo.expect_count_unread_emails().returning(|| Ok(3));
        let service = EmailsService::new(repo);

        assert_eq!(service.count_unread_emails().await.unwrap(), 3);
    }

    #[tokio::test]
    async fn delete_email_succeeds() {
        let mut repo = MockEmailRepositoryExt::new();
        repo.expect_delete_email().returning(|_| Ok(()));
        let service = EmailsService::new(repo);

        assert!(service.delete_email("em-001").await.is_ok());
    }
}
