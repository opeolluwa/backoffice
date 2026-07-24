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

    async fn update_app_config(
        &self,
        app_name: Option<String>,
        support_email: Option<String>,
        default_currency: Option<String>,
        default_language: Option<String>,
        maintenance_mode: Option<bool>,
        brand_color: Option<String>,
        logo_url: Option<String>,
    ) -> Result<app_config::Model, ServiceError>;
}

impl<R: AppConfigRepositoryExt + Send + Sync> AppConfigServiceExt for AppConfigService<R> {
    async fn get_app_config(&self) -> Result<Option<app_config::Model>, ServiceError> {
        Ok(self.repo.find_app_config().await?)
    }

    async fn update_app_config(
        &self,
        app_name: Option<String>,
        support_email: Option<String>,
        default_currency: Option<String>,
        default_language: Option<String>,
        maintenance_mode: Option<bool>,
        brand_color: Option<String>,
        logo_url: Option<String>,
    ) -> Result<app_config::Model, ServiceError> {
        let existing = self.repo.find_app_config().await?;

        match existing {
            Some(_) => Ok(self
                .repo
                .update_app_config(
                    app_name,
                    support_email,
                    default_currency,
                    default_language,
                    maintenance_mode,
                    brand_color,
                    logo_url,
                )
                .await?),
            None => {
                let id = Ulid::new().to_string();
                Ok(self
                    .repo
                    .create_app_config(
                        &id,
                        app_name,
                        support_email,
                        default_currency,
                        default_language,
                        brand_color,
                        logo_url,
                    )
                    .await?)
            }
        }
    }
}
