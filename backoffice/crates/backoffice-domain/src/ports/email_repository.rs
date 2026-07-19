use crate::{
    dto::{CreateEmailCommand, UpdateEmailCommand},
    errors::database_error::DatabaseError,
    models::emails,
};

#[cfg_attr(test, mockall::automock)]
#[allow(async_fn_in_trait)]
pub trait EmailRepositoryExt {
    async fn create_email(
        &self,
        command: &CreateEmailCommand,
        user_identifier: &str,
    ) -> Result<emails::Model, DatabaseError>;

    async fn find_email_by_identifier(
        &self,
        identifier: &str,
        user_identifier: &str,
    ) -> Result<emails::Model, DatabaseError>;

    async fn find_all_emails(
        &self,
        user_identifier: &str,
    ) -> Result<Vec<emails::Model>, DatabaseError>;

    async fn find_emails_by_tag(
        &self,
        tag: &str,
        user_identifier: &str,
    ) -> Result<Vec<emails::Model>, DatabaseError>;

    async fn find_starred_emails(
        &self,
        user_identifier: &str,
    ) -> Result<Vec<emails::Model>, DatabaseError>;

    async fn find_unread_emails(
        &self,
        user_identifier: &str,
    ) -> Result<Vec<emails::Model>, DatabaseError>;

    async fn update_email(
        &self,
        identifier: &str,
        command: &UpdateEmailCommand,
        user_identifier: &str,
    ) -> Result<emails::Model, DatabaseError>;

    async fn delete_email(
        &self,
        identifier: &str,
        user_identifier: &str,
    ) -> Result<(), DatabaseError>;

    async fn count_emails(&self, user_identifier: &str) -> Result<i64, DatabaseError>;

    async fn count_unread_emails(&self, user_identifier: &str) -> Result<i64, DatabaseError>;
}
