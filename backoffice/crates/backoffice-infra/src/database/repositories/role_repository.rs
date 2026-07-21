use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set};

use backoffice_domain::{
    errors::database_error::DatabaseError,
    models::user_roles::{self, Entity as RoleEntity},
    ports::role_repository::RoleRepositoryExt,
};

use crate::database::repositories::base::Repository;

#[derive(Clone)]
pub struct RoleRepository {
    db: DatabaseConnection,
}

impl Repository for RoleRepository {
    fn init(db: &DatabaseConnection) -> Self {
        Self { db: db.clone() }
    }
}

impl RoleRepositoryExt for RoleRepository {
    async fn find_role_by_identifier(
        &self,
        identifier: &str,
    ) -> Result<Option<user_roles::Model>, DatabaseError> {
        RoleEntity::find_by_id(identifier)
            .one(&self.db)
            .await
            .map_err(|e| DatabaseError::OperationFailed(e.to_string()))
    }

    async fn find_role_by_name(
        &self,
        name: &str,
    ) -> Result<Option<user_roles::Model>, DatabaseError> {
        RoleEntity::find()
            .filter(user_roles::Column::Name.eq(name))
            .one(&self.db)
            .await
            .map_err(|e| DatabaseError::OperationFailed(e.to_string()))
    }

    async fn create_role(
        &self,
        identifier: &str,
        name: &str,
        description: Option<String>,
    ) -> Result<user_roles::Model, DatabaseError> {
        let model = user_roles::ActiveModel {
            identifier: Set(identifier.to_string()),
            name: Set(name.to_string()),
            description: Set(description),
            ..Default::default()
        };
        model
            .insert(&self.db)
            .await
            .map_err(|e| DatabaseError::OperationFailed(e.to_string()))
    }
}
