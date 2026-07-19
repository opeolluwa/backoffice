use crate::{
    dto::SaveProductCommand,
    models::products,
    errors::database_error::DatabaseError,
};

#[cfg_attr(test, mockall::automock)]
#[allow(async_fn_in_trait)]
pub trait ProductRepositoryExt {
    async fn create_product(
        &self,
        command: &SaveProductCommand,
        user_identifier: &str,
        marketplace_identifier: &str,
    ) -> Result<products::Model, DatabaseError>;

    async fn retrieve_product(
        &self,
        identifier: &str,
        user_identifier: &str,
    ) -> Result<products::Model, DatabaseError>;
}
