use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set};
use ulid::Ulid;

use crate::errors::database_error::DatabaseError;
use crate::{
    dto::SaveProductCommand,
    models::products::{self, Entity as ProductEntity},
};

use crate::repositories::base::Repository;

#[cfg_attr(test, mockall::automock)]
#[allow(async_fn_in_trait)]
pub trait ProductRepositoryExt {
    async fn create_product(
        &self,
        command: &SaveProductCommand,
    ) -> Result<products::Model, DatabaseError>;

    async fn find_all_products(&self) -> Result<Vec<products::Model>, DatabaseError>;

    async fn retrieve_product(&self, identifier: &str) -> Result<products::Model, DatabaseError>;

    async fn find_product_by_identifier(
        &self,
        identifier: &str,
    ) -> Result<products::Model, DatabaseError>;
}

#[derive(Debug, Clone)]
pub struct ProductRepository {
    db: DatabaseConnection,
}

impl Repository for ProductRepository {
    fn init(db: &DatabaseConnection) -> Self {
        Self { db: db.clone() }
    }
}

impl ProductRepositoryExt for ProductRepository {
    async fn create_product(
        &self,
        command: &SaveProductCommand,
    ) -> Result<products::Model, DatabaseError> {
        let model = products::ActiveModel {
            identifier: Set(Ulid::new().to_string()),
            name: Set(command.name.clone()),
            picture: Set(command.picture.clone()),
            price: Set(command.price),
            description: Set(command.description.clone()),
            currency_identifier: Set(Some(command.currency_identifier.clone())),
            ..Default::default()
        };
        model.insert(&self.db).await.map_err(DatabaseError::from)
    }

    async fn find_all_products(&self) -> Result<Vec<products::Model>, DatabaseError> {
        ProductEntity::find()
            .all(&self.db)
            .await
            .map_err(DatabaseError::from)
    }

    async fn retrieve_product(&self, identifier: &str) -> Result<products::Model, DatabaseError> {
        ProductEntity::find()
            .filter(products::Column::Identifier.eq(identifier))
            .one(&self.db)
            .await
            .map_err(DatabaseError::from)?
            .ok_or_else(|| DatabaseError::NotFound("product not found".to_string()))
    }

    async fn find_product_by_identifier(
        &self,
        identifier: &str,
    ) -> Result<products::Model, DatabaseError> {
        ProductEntity::find()
            .filter(products::Column::Identifier.eq(identifier))
            .one(&self.db)
            .await
            .map_err(DatabaseError::from)?
            .ok_or_else(|| DatabaseError::NotFound("product not found".to_string()))
    }
}
