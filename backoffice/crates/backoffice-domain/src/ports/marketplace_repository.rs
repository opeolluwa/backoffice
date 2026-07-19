use crate::{
    dto::CreateMarketplaceCommand, errors::database_error::DatabaseError, models::marketplaces,
};

#[cfg_attr(test, mockall::automock)]
#[allow(async_fn_in_trait)]
pub trait MarketplaceRepositoryExt {
    async fn create_marketplace(
        &self,
        command: &CreateMarketplaceCommand,
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
        command: &CreateMarketplaceCommand,
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
