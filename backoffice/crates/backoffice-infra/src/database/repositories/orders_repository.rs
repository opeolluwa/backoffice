use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, PaginatorTrait, QueryFilter,
    Set,
};
use ulid::Ulid;

use backoffice_domain::errors::database_error::DatabaseError;
use backoffice_domain::{
    dto::CreateOrdersCommand,
    models::orders::{self, Entity as OrdersEntity},
    ports::orders_repository::OrdersRepositoryExt,
};

use crate::database::repositories::base::Repository;

#[derive(Debug, Clone)]
pub struct OrdersRepository {
    db: DatabaseConnection,
}

impl Repository for OrdersRepository {
    fn init(db: &DatabaseConnection) -> Self {
        Self { db: db.clone() }
    }
}

impl OrdersRepositoryExt for OrdersRepository {
    async fn create_orders(
        &self,
        command: &CreateOrdersCommand,
    ) -> Result<orders::Model, DatabaseError> {
        let model = orders::ActiveModel {
            identifier: Set(Ulid::new().to_string()),
            ..Default::default()
        };
        let result = model.insert(&self.db).await.map_err(DatabaseError::from)?;
        Ok(result)
    }

    async fn find_orders_by_identifier(
        &self,
        identifier: &str,
    ) -> Result<orders::Model, DatabaseError> {
        OrdersEntity::find()
            .filter(orders::Column::Identifier.eq(identifier))
            .one(&self.db)
            .await
            .map_err(DatabaseError::from)?
            .ok_or_else(|| DatabaseError::NotFound("orders not found".to_string()))
    }

    async fn find_all_orders(&self) -> Result<Vec<orders::Model>, DatabaseError> {
        OrdersEntity::find()
            .all(&self.db)
            .await
            .map_err(DatabaseError::from)
    }

    async fn update_orders_by_identifier(
        &self,
        identifier: &str,
        command: &CreateOrdersCommand,
    ) -> Result<orders::Model, DatabaseError> {
        let existing = OrdersEntity::find()
            .filter(orders::Column::Identifier.eq(identifier))
            .one(&self.db)
            .await
            .map_err(DatabaseError::from)?
            .ok_or_else(|| DatabaseError::NotFound("orders not found".to_string()))?;

        let mut active: orders::ActiveModel = existing.into();
        //TODO: active.updated_at = Set(Some(chrono::Dat)));

        active.update(&self.db).await.map_err(DatabaseError::from)
    }

    async fn delete_orders_by_identifier(
        &self,
        identifier: &str,
    ) -> Result<(), DatabaseError> {
        OrdersEntity::delete_many()
            .filter(orders::Column::Identifier.eq(identifier))
            .exec(&self.db)
            .await
            .map_err(DatabaseError::from)?;
        Ok(())
    }

    async fn count_orders(&self) -> Result<i64, DatabaseError> {
        let count = OrdersEntity::find()
            .count(&self.db)
            .await
            .map_err(DatabaseError::from)?;
        Ok(count as i64)
    }
}
