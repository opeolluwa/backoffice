use rust_decimal::Decimal;
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set};
use ulid::Ulid;

use backoffice_domain::errors::database_error::DatabaseError;
use backoffice_domain::{
    dto::SaveProductCommand,
    models::products::{self, Entity as ProductEntity},
    ports::product_repository::ProductRepositoryExt,
};

use crate::database::repositories::base::Repository;

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
        user_identifier: &str,
        marketplace_identifier: &str,
    ) -> Result<products::Model, DatabaseError> {
        let model = products::ActiveModel {
            identifier: Set(Ulid::new().to_string()),
            name: Set(command.name.clone()),
            picture: Set(Some(command.picture.clone())),
            price: Set(Decimal::from(command.price)),
            description: Set(command.description.clone()),
            created_by_identifier: Set(Some(user_identifier.to_string())),
            marketplace_identifier: Set(Some(marketplace_identifier.to_string())),
            currency_identifier: Set(Some(command.currency_identifier.clone())),
            ..Default::default()
        };
        model.insert(&self.db).await.map_err(DatabaseError::from)
    }

    async fn retrieve_product(
        &self,
        identifier: &str,
        user_identifier: &str,
    ) -> Result<products::Model, DatabaseError> {
        ProductEntity::find()
            .filter(products::Column::Identifier.eq(identifier))
            .filter(products::Column::CreatedByIdentifier.eq(user_identifier))
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
