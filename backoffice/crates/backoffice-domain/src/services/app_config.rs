use ulid::Ulid;

use crate::{
    errors::service_error::ServiceError, models::app_config,
    ports::app_config_repository::AppConfigRepositoryExt,
};

#[derive(Clone)]
pub struct AppConfigService<R: AppConfigRepositoryExt> {
    repo: R,
}

impl<R: AppConfigRepositoryExt> AppConfigService<R> {
    pub fn new(repo: R) -> Self {
        Self { repo }
    }
}

pub trait AppConfigServiceExt {
    async fn get_app_config(&self) -> Result<Option<app_config::Model>, ServiceError>;

    async fn create_app_config(
        &self,
        app_name: Option<String>,
        support_email: Option<String>,
    ) -> Result<app_config::Model, ServiceError>;

    async fn update_app_config(
        &self,
        default_currency: Option<String>,
        default_language: Option<String>,
    ) -> Result<app_config::Model, ServiceError>;
}

impl<R: AppConfigRepositoryExt + Send + Sync> AppConfigServiceExt for AppConfigService<R> {
    async fn get_app_config(&self) -> Result<Option<app_config::Model>, ServiceError> {
        Ok(self.repo.find_app_config_by_identifier("1").await?)
    }

    async fn create_app_config(
        &self,
        app_name: Option<String>,
        support_email: Option<String>,
    ) -> Result<app_config::Model, ServiceError> {
        Ok(self
            .repo
            .create_app_config(
                &Ulid::new().to_string(),
                app_name,
                support_email,
                None,
                None,
            )
            .await?)
    }

    async fn update_app_config(
        &self,
        default_currency: Option<String>,
        default_language: Option<String>,
    ) -> Result<app_config::Model, ServiceError> {
        let config = self.repo.find_app_config_by_identifier("1").await?;
        let id = config
            .as_ref()
            .map(|c| c.identifier.as_str())
            .unwrap_or("1");

        Ok(self
            .repo
            .update_app_config(id, default_currency, default_language)
            .await?)
    }
}
