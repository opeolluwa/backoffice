use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set};

use backoffice_domain::{
    errors::database_error::DatabaseError,
    models::app_config::{self, Entity as AppConfigEntity},
    ports::app_config_repository::AppConfigRepositoryExt,
};

use crate::database::repositories::base::Repository;

#[derive(Clone)]
pub struct AppConfigRepository {
    db: DatabaseConnection,
}

impl Repository for AppConfigRepository {
    fn init(db: &DatabaseConnection) -> Self {
        Self { db: db.clone() }
    }
}

impl AppConfigRepositoryExt for AppConfigRepository {
    async fn find_app_config_by_identifier(
        &self,
        identifier: &str,
    ) -> Result<Option<app_config::Model>, DatabaseError> {
        AppConfigEntity::find_by_id(identifier)
            .one(&self.db)
            .await
            .map_err(|e| DatabaseError::OperationFailed(e.to_string()))
    }

    async fn create_app_config(
        &self,
        identifier: &str,
        app_name: Option<String>,
        support_email: Option<String>,
        default_currency: Option<String>,
        default_language: Option<String>,
    ) -> Result<app_config::Model, DatabaseError> {
        let model = app_config::ActiveModel {
            identifier: Set(identifier.to_string()),
            app_name: Set(app_name),
            maintenance_mode: Set(false),
            support_email: Set(support_email),
            default_currency: Set(default_currency),
            default_language: Set(default_language),
            ..Default::default()
        };
        model
            .insert(&self.db)
            .await
            .map_err(|e| DatabaseError::OperationFailed(e.to_string()))
    }

    async fn update_app_config(
        &self,
        identifier: &str,
        default_currency: Option<String>,
        default_language: Option<String>,
    ) -> Result<app_config::Model, DatabaseError> {
        let config = AppConfigEntity::find()
            .filter(app_config::Column::Identifier.eq(identifier))
            .one(&self.db)
            .await
            .map_err(|e| DatabaseError::OperationFailed(e.to_string()))?
            .ok_or_else(|| DatabaseError::OperationFailed("App config not found".to_string()))?;

        let mut active: app_config::ActiveModel = config.into();
        active.default_currency = Set(default_currency);
        active.default_language = Set(default_language);

        active
            .update(&self.db)
            .await
            .map_err(|e| DatabaseError::OperationFailed(e.to_string()))
    }
}
