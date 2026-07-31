use crate::{errors::database_error::DatabaseError, models::app_config};

#[cfg_attr(test, mockall::automock)]
#[allow(async_fn_in_trait)]
pub trait AppConfigRepositoryExt {
    async fn find_app_config(&self) -> Result<Option<app_config::Model>, DatabaseError>;

    async fn update_app_config(
        &self,
        app_name: Option<String>,
        support_email: Option<String>,
        default_currency: Option<String>,
        default_language: Option<String>,
        maintenance_mode: Option<bool>,
        logo_url: Option<String>,
    ) -> Result<app_config::Model, DatabaseError>;

    async fn create_app_config(
        &self,
        identifier: &str,
        app_name: Option<String>,
        support_email: Option<String>,
        default_currency: Option<String>,
        default_language: Option<String>,
        logo_url: Option<String>,
    ) -> Result<app_config::Model, DatabaseError>;
}
