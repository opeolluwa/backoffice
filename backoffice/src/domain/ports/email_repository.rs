use crate::{
    api::http::extractors::email::{CreateEmailRequest, UpdateEmailRequest},
    entities::emails,
    errors::database_error::DatabaseError,
};

pub(crate) trait EmailRepositoryExt {
    async fn create_email(
        &self,
        request: &CreateEmailRequest,
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
        request: &UpdateEmailRequest,
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
