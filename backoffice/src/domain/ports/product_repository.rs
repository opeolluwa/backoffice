use crate::{
    adapters::requests::products::SaveProductRequest,
    entities::products,
    errors::database_error::DatabaseError,
};

pub(crate) trait ProductRepositoryExt {
    async fn create_product(
        &self,
        request: &SaveProductRequest,
        user_identifier: &str,
        marketplace_identifier: &str,
    ) -> Result<products::Model, DatabaseError>;

    async fn retrieve_product(
        &self,
        identifier: &str,
        user_identifier: &str,
    ) -> Result<products::Model, DatabaseError>;
}
