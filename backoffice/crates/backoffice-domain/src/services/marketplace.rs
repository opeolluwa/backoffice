use crate::{
    dto::CreateMarketplaceCommand,
    models::marketplaces,
    ports::marketplace_repository::MarketplaceRepositoryExt,
};
use crate::errors::service_error::ServiceError;

pub struct MarketplaceService<R: MarketplaceRepositoryExt> {
    repo: R,
}

impl<R: MarketplaceRepositoryExt> MarketplaceService<R> {
    pub fn new(repo: R) -> Self {
        Self { repo }
    }
}

pub trait MarketplaceServiceExt {
    async fn create_marketplace(
        &self,
        command: &CreateMarketplaceCommand,
        user_identifier: &str,
    ) -> Result<marketplaces::Model, ServiceError>;

    async fn find_marketplace_by_identifier(
        &self,
        identifier: &str,
        user_identifier: &str,
    ) -> Result<marketplaces::Model, ServiceError>;

    async fn find_all_marketplaces(
        &self,
        user_identifier: &str,
    ) -> Result<Vec<marketplaces::Model>, ServiceError>;

    async fn update_marketplace_by_identifier(
        &self,
        identifier: &str,
        command: &CreateMarketplaceCommand,
        user_identifier: &str,
    ) -> Result<marketplaces::Model, ServiceError>;

    async fn delete_marketplace_by_identifier(
        &self,
        identifier: &str,
        user_identifier: &str,
    ) -> Result<(), ServiceError>;

    async fn count_marketplaces(&self, user_identifier: &str) -> Result<i64, ServiceError>;
}

impl<R: MarketplaceRepositoryExt + Send + Sync> MarketplaceServiceExt for MarketplaceService<R> {
    async fn create_marketplace(
        &self,
        command: &CreateMarketplaceCommand,
        user_identifier: &str,
    ) -> Result<marketplaces::Model, ServiceError> {
        Ok(self.repo.create_marketplace(command, user_identifier).await?)
    }

    async fn find_marketplace_by_identifier(
        &self,
        identifier: &str,
        user_identifier: &str,
    ) -> Result<marketplaces::Model, ServiceError> {
        Ok(self
            .repo
            .find_marketplace_by_identifier(identifier, user_identifier)
            .await?)
    }

    async fn find_all_marketplaces(
        &self,
        user_identifier: &str,
    ) -> Result<Vec<marketplaces::Model>, ServiceError> {
        Ok(self.repo.find_all_marketplaces(user_identifier).await?)
    }

    async fn update_marketplace_by_identifier(
        &self,
        identifier: &str,
        command: &CreateMarketplaceCommand,
        user_identifier: &str,
    ) -> Result<marketplaces::Model, ServiceError> {
        Ok(self
            .repo
            .update_marketplace_by_identifier(identifier, command, user_identifier)
            .await?)
    }

    async fn delete_marketplace_by_identifier(
        &self,
        identifier: &str,
        user_identifier: &str,
    ) -> Result<(), ServiceError> {
        Ok(self
            .repo
            .delete_marketplace_by_identifier(identifier, user_identifier)
            .await?)
    }

    async fn count_marketplaces(&self, user_identifier: &str) -> Result<i64, ServiceError> {
        Ok(self.repo.count_marketplaces(user_identifier).await?)
    }
}
