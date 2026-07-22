use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, PaginatorTrait, QueryFilter,
    Set, TransactionTrait,
};
use ulid::Ulid;

use backoffice_domain::errors::database_error::DatabaseError;
use backoffice_domain::{
    dto::PlaceOrderCommand,
    models::{
        orders::{self, Entity as OrdersEntity},
        products::{self, Entity as ProductEntity},
        sea_orm_active_enums::OrderStatus,
    },
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
    async fn place_orders(
        &self,
        command: &PlaceOrderCommand,
    ) -> Result<Vec<(orders::Model, products::Model)>, DatabaseError> {
        let txn = self.db.begin().await.map_err(DatabaseError::from)?;
        let mut created_identifiers = Vec::new();

        for item in &command.items {
            let product = ProductEntity::find()
                .filter(products::Column::Identifier.eq(&item.product_identifier))
                .one(&txn)
                .await
                .map_err(DatabaseError::from)?
                .ok_or_else(|| {
                    DatabaseError::NotFound(format!(
                        "product not found: {}",
                        item.product_identifier
                    ))
                })?;

            let model = orders::ActiveModel {
                identifier: Set(Ulid::new().to_string()),
                product_identifier: Set(product.identifier),
                quantity: Set(item.quantity),
                status: Set(Some(OrderStatus::Pending)),
                ..Default::default()
            };

            let result = model.insert(&txn).await.map_err(DatabaseError::from)?;
            created_identifiers.push(result.identifier);
        }

        let results = OrdersEntity::find()
            .filter(orders::Column::Identifier.is_in(created_identifiers))
            .find_also_related(ProductEntity)
            .all(&txn)
            .await
            .map_err(DatabaseError::from)?;

        txn.commit().await.map_err(DatabaseError::from)?;

        let pairs = results
            .into_iter()
            .filter_map(|(order, product)| product.map(|p| (order, p)))
            .collect();

        Ok(pairs)
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
        command: &PlaceOrderCommand,
    ) -> Result<orders::Model, DatabaseError> {
        let existing = OrdersEntity::find()
            .filter(orders::Column::Identifier.eq(identifier))
            .one(&self.db)
            .await
            .map_err(DatabaseError::from)?
            .ok_or_else(|| DatabaseError::NotFound("orders not found".to_string()))?;

        let mut active: orders::ActiveModel = existing.into();
        if let Some(first) = command.items.first() {
            active.product_identifier = Set(first.product_identifier.clone());
            active.quantity = Set(first.quantity);
        }

        active.update(&self.db).await.map_err(DatabaseError::from)
    }

    async fn delete_orders_by_identifier(&self, identifier: &str) -> Result<(), DatabaseError> {
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
