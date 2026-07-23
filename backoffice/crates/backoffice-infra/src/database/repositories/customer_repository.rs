use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, PaginatorTrait, QueryFilter};

use backoffice_domain::errors::database_error::DatabaseError;
use backoffice_domain::{
    models::customers::{self, Entity as CustomerEntity},
    ports::customer_repository::CustomerRepositoryExt,
};

use crate::database::repositories::base::Repository;

#[derive(Debug, Clone)]
pub struct CustomerRepository {
    db: DatabaseConnection,
}

impl Repository for CustomerRepository {
    fn init(db: &DatabaseConnection) -> Self {
        Self { db: db.clone() }
    }
}

impl CustomerRepositoryExt for CustomerRepository {
    async fn find_all_customers(&self) -> Result<Vec<customers::Model>, DatabaseError> {
        CustomerEntity::find()
            .all(&self.db)
            .await
            .map_err(DatabaseError::from)
    }

    async fn find_customer_by_identifier(
        &self,
        identifier: &str,
    ) -> Result<customers::Model, DatabaseError> {
        CustomerEntity::find()
            .filter(customers::Column::Identifier.eq(identifier))
            .one(&self.db)
            .await
            .map_err(DatabaseError::from)?
            .ok_or_else(|| DatabaseError::NotFound("customer not found".to_string()))
    }

    async fn delete_customer_by_identifier(&self, identifier: &str) -> Result<(), DatabaseError> {
        CustomerEntity::delete_many()
            .filter(customers::Column::Identifier.eq(identifier))
            .exec(&self.db)
            .await
            .map_err(DatabaseError::from)?;
        Ok(())
    }

    async fn count_customers(&self) -> Result<i64, DatabaseError> {
        let count = CustomerEntity::find()
            .count(&self.db)
            .await
            .map_err(DatabaseError::from)?;
        Ok(count as i64)
    }
}
