use crate::adapters::requests::marketplace::CreateMarketplaceRequest;
use crate::entities::marketplace::MarketPlace;
use crate::errors::service_error::ServiceError;
use crate::repositories::base::Repository;
use sqlx::PgPool;

use crate::repositories::marketplace_repository::MarketplaceRepository;
use crate::repositories::marketplace_repository::MarketplaceRepositoryExt;

#[derive(Clone)]
pub struct MarketplaceService {
    marketplace_repository: MarketplaceRepository,
}

impl MarketplaceService {
    pub fn init(pool: &PgPool) -> Self {
        Self {
            marketplace_repository: MarketplaceRepository::init(pool),
        }
    }
}

pub(crate) trait MarketplaceServiceExt {
    async fn create_marketplace(
        &self,
        request: &CreateMarketplaceRequest,
        user_identifier: &str,
    ) -> Result<MarketPlace, ServiceError>;

    async fn find_marketplace_by_identifier(
        &self,
        identifier: &str,
        user_identifier: &str,
    ) -> Result<MarketPlace, ServiceError>;

    async fn find_all_marketplaces(
        &self,
        user_identifier: &str,
    ) -> Result<Vec<MarketPlace>, ServiceError>;

    async fn update_marketplace_by_identifier(
        &self,
        identifier: &str,
        request: &CreateMarketplaceRequest,
        user_identifier: &str,
    ) -> Result<MarketPlace, ServiceError>;

    async fn delete_marketplace_by_identifier(
        &self,
        identifier: &str,
        user_identifier: &str,
    ) -> Result<(), ServiceError>;

    async fn count_marketplaces(&self, user_identifier: &str) -> Result<i64, ServiceError>;
}

impl MarketplaceServiceExt for MarketplaceService {
    async fn create_marketplace(
        &self,
        request: &CreateMarketplaceRequest,
        user_identifier: &str,
    ) -> Result<MarketPlace, ServiceError> {
        self.marketplace_repository
            .create_marketplace(request, user_identifier)
            .await
            .map_err(|e| ServiceError::OperationFailed(e.to_string()))
    }

    async fn find_marketplace_by_identifier(
        &self,
        identifier: &str,
        user_identifier: &str,
    ) -> Result<MarketPlace, ServiceError> {
        self.marketplace_repository
            .find_marketplace_by_identifier(identifier, user_identifier)
            .await
            .map_err(|e| ServiceError::OperationFailed(e.to_string()))
    }

    async fn find_all_marketplaces(
        &self,
        user_identifier: &str,
    ) -> Result<Vec<MarketPlace>, ServiceError> {
        self.marketplace_repository
            .find_all_marketplaces(user_identifier)
            .await
            .map_err(|e| ServiceError::OperationFailed(e.to_string()))
    }

    async fn update_marketplace_by_identifier(
        &self,
        identifier: &str,
        request: &CreateMarketplaceRequest,
        user_identifier: &str,
    ) -> Result<MarketPlace, ServiceError> {
        self.marketplace_repository
            .update_marketplace_by_identifier(identifier, request, user_identifier)
            .await
            .map_err(|e| ServiceError::OperationFailed(e.to_string()))
    }

    async fn delete_marketplace_by_identifier(
        &self,
        identifier: &str,
        user_identifier: &str,
    ) -> Result<(), ServiceError> {
        self.marketplace_repository
            .delete_marketplace_by_identifier(identifier, user_identifier)
            .await
            .map_err(|e| ServiceError::OperationFailed(e.to_string()))
    }

    async fn count_marketplaces(&self, user_identifier: &str) -> Result<i64, ServiceError> {
        self.marketplace_repository
            .count_marketplaces(user_identifier)
            .await
            .map_err(|e| ServiceError::OperationFailed(e.to_string()))
    }
}
