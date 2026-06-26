use crate::adapters::requests::marketplace::CreateMarketplaceRequest;
use crate::domain::ports::marketplace_repository::MarketplaceRepositoryExt;
use crate::entities::marketplaces;
use crate::errors::service_error::ServiceError;

#[derive(Clone)]
pub struct MarketplaceService<R: MarketplaceRepositoryExt> {
    repo: R,
}

impl<R: MarketplaceRepositoryExt + Clone> MarketplaceService<R> {
    pub fn new(repo: R) -> Self {
        Self { repo }
    }
}

pub(crate) trait MarketplaceServiceExt {
    async fn create_marketplace(
        &self,
        request: &CreateMarketplaceRequest,
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
        request: &CreateMarketplaceRequest,
        user_identifier: &str,
    ) -> Result<marketplaces::Model, ServiceError>;

    async fn delete_marketplace_by_identifier(
        &self,
        identifier: &str,
        user_identifier: &str,
    ) -> Result<(), ServiceError>;

    async fn count_marketplaces(&self, user_identifier: &str) -> Result<i64, ServiceError>;
}

impl<R: MarketplaceRepositoryExt + Clone + Send + Sync> MarketplaceServiceExt
    for MarketplaceService<R>
{
    async fn create_marketplace(
        &self,
        request: &CreateMarketplaceRequest,
        user_identifier: &str,
    ) -> Result<marketplaces::Model, ServiceError> {
        self.repo
            .create_marketplace(request, user_identifier)
            .await
            .map_err(|e| ServiceError::OperationFailed(e.to_string()))
    }

    async fn find_marketplace_by_identifier(
        &self,
        identifier: &str,
        user_identifier: &str,
    ) -> Result<marketplaces::Model, ServiceError> {
        self.repo
            .find_marketplace_by_identifier(identifier, user_identifier)
            .await
            .map_err(|e| ServiceError::OperationFailed(e.to_string()))
    }

    async fn find_all_marketplaces(
        &self,
        user_identifier: &str,
    ) -> Result<Vec<marketplaces::Model>, ServiceError> {
        self.repo
            .find_all_marketplaces(user_identifier)
            .await
            .map_err(|e| ServiceError::OperationFailed(e.to_string()))
    }

    async fn update_marketplace_by_identifier(
        &self,
        identifier: &str,
        request: &CreateMarketplaceRequest,
        user_identifier: &str,
    ) -> Result<marketplaces::Model, ServiceError> {
        self.repo
            .update_marketplace_by_identifier(identifier, request, user_identifier)
            .await
            .map_err(|e| ServiceError::OperationFailed(e.to_string()))
    }

    async fn delete_marketplace_by_identifier(
        &self,
        identifier: &str,
        user_identifier: &str,
    ) -> Result<(), ServiceError> {
        self.repo
            .delete_marketplace_by_identifier(identifier, user_identifier)
            .await
            .map_err(|e| ServiceError::OperationFailed(e.to_string()))
    }

    async fn count_marketplaces(&self, user_identifier: &str) -> Result<i64, ServiceError> {
        self.repo
            .count_marketplaces(user_identifier)
            .await
            .map_err(|e| ServiceError::OperationFailed(e.to_string()))
    }
}
