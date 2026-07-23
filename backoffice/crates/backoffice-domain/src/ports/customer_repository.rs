use crate::{errors::database_error::DatabaseError, models::customers};

#[cfg_attr(test, mockall::automock)]
#[allow(async_fn_in_trait)]
pub trait CustomerRepositoryExt {
    async fn find_all_customers(&self) -> Result<Vec<customers::Model>, DatabaseError>;

    async fn find_customer_by_identifier(
        &self,
        identifier: &str,
    ) -> Result<customers::Model, DatabaseError>;

    async fn delete_customer_by_identifier(&self, identifier: &str) -> Result<(), DatabaseError>;

    async fn count_customers(&self) -> Result<i64, DatabaseError>;
}
