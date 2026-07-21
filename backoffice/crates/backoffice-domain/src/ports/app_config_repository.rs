use crate::{errors::database_error::DatabaseError, models::app_config};

#[cfg_attr(test, mockall::automock)]
#[allow(async_fn_in_trait)]
pub trait AppConfigRepositoryExt {
    async fn find_app_config_by_identifier(
        &self,
        identifier: i16,
    ) -> Result<Option<app_config::Model>, DatabaseError>;

    async fn create_app_config(
        &self,
        identifier: i16,
        app_name: Option<String>,
        support_email: Option<String>,
    ) -> Result<app_config::Model, DatabaseError>;
}
