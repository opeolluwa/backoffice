use crate::{
    adapters::requests::marketplace::CreateMarketplaceRequest,
    entities::marketplaces,
    errors::database_error::DatabaseError,
};

pub(crate) trait MarketplaceRepositoryExt {
    async fn create_marketplace(
        &self,
        request: &CreateMarketplaceRequest,
        user_identifier: &str,
    ) -> Result<marketplaces::Model, DatabaseError>;

    async fn find_marketplace_by_identifier(
        &self,
        identifier: &str,
        user_identifier: &str,
    ) -> Result<marketplaces::Model, DatabaseError>;

    #[allow(dead_code)]
    async fn find_marketplace_by_name(
        &self,
        name: &str,
        user_identifier: &str,
    ) -> Result<marketplaces::Model, DatabaseError>;

    async fn find_all_marketplaces(
        &self,
        user_identifier: &str,
    ) -> Result<Vec<marketplaces::Model>, DatabaseError>;

    async fn update_marketplace_by_identifier(
        &self,
        identifier: &str,
        request: &CreateMarketplaceRequest,
        user_identifier: &str,
    ) -> Result<marketplaces::Model, DatabaseError>;

    async fn delete_marketplace_by_identifier(
        &self,
        identifier: &str,
        user_identifier: &str,
    ) -> Result<(), DatabaseError>;

    #[allow(dead_code)]
    async fn marketplace_exists(
        &self,
        identifier: &str,
        user_identifier: &str,
    ) -> Result<bool, DatabaseError>;

    async fn count_marketplaces(&self, user_identifier: &str) -> Result<i64, DatabaseError>;
}
