use crate::{dto::SaveProductCommand, errors::database_error::DatabaseError, models::products};

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
