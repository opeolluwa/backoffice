use crate::{errors::database_error::DatabaseError, models::app_config};

#[cfg_attr(test, mockall::automock)]
#[allow(async_fn_in_trait)]
pub trait AppConfigRepositoryExt {
    async fn find_app_config_by_identifier(
        &self,
        identifier: &str,
    ) -> Result<Option<app_config::Model>, DatabaseError>;

    async fn create_app_config(
        &self,
        identifier: &str,
        app_name: Option<String>,
        support_email: Option<String>,
        default_currency: Option<String>,
        default_language: Option<String>,
    ) -> Result<app_config::Model, DatabaseError>;

    async fn update_app_config(
        &self,
        identifier: &str,
        default_currency: Option<String>,
        default_language: Option<String>,
    ) -> Result<app_config::Model, DatabaseError>;
}
