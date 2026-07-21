use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, Set};

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
        identifier: i16,
    ) -> Result<Option<app_config::Model>, DatabaseError> {
        AppConfigEntity::find_by_id(identifier)
            .one(&self.db)
            .await
            .map_err(|e| DatabaseError::OperationFailed(e.to_string()))
    }

    async fn create_app_config(
        &self,
        identifier: i16,
        app_name: Option<String>,
        support_email: Option<String>,
    ) -> Result<app_config::Model, DatabaseError> {
        let model = app_config::ActiveModel {
            identifier: Set(identifier),
            app_name: Set(app_name),
            maintenance_mode: Set(false),
            support_email: Set(support_email),
            ..Default::default()
        };
        model
            .insert(&self.db)
            .await
            .map_err(|e| DatabaseError::OperationFailed(e.to_string()))
    }
}
