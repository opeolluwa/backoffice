//! Marketplace repository module
//! The Marketplace contains different products, see it as a catalogue of items.
//!
use std::sync::Arc;

use sqlx::PgPool;
use ulid::Ulid;

use crate::{
    adapters::requests::marketplace::CreateMarketplaceRequest, entities::marketplace::MarketPlace,
    errors::repository_error::RepositoryError, repositories::base::Repository,
};

#[derive(Debug, Clone)]
pub struct MarketplaceRepository {
    pool: Arc<PgPool>,
}

impl Repository for MarketplaceRepository {
    fn init(pool: &PgPool) -> Self {
        Self {
            pool: Arc::new(pool.clone()),
        }
    }
}

pub(crate) trait MarketplaceRepositoryExt {
    async fn create_marketplace(
        &self,
        request: &CreateMarketplaceRequest,
        user_identifier: &str,
    ) -> Result<MarketPlace, RepositoryError>;

    async fn find_marketplace_by_identifier(
        &self,
        identifier: &str,
        user_identifier: &str,
    ) -> Result<MarketPlace, RepositoryError>;

    async fn find_marketplace_by_name(
        &self,
        identifier: &str,
        user_identifier: &str,
    ) -> Result<MarketPlace, RepositoryError>;

    async fn find_all_marketplaces(
        &self,
        user_identifier: &str,
    ) -> Result<Vec<MarketPlace>, RepositoryError>;

    async fn update_marketplace_by_identifier(
        &self,
        identifier: &str,
        request: &CreateMarketplaceRequest,
        user_identifier: &str,
    ) -> Result<MarketPlace, RepositoryError>;

    async fn delete_marketplace_by_identifier(
        &self,
        identifier: &str,
        user_identifier: &str,
    ) -> Result<(), RepositoryError>;

    async fn marketplace_exists(
        &self,
        identifier: &str,
        user_identifier: &str,
    ) -> Result<bool, RepositoryError>;

    async fn count_marketplaces(&self, user_identifier: &str) -> Result<i64, RepositoryError>;
}

impl MarketplaceRepositoryExt for MarketplaceRepository {
    async fn create_marketplace(
        &self,
        request: &CreateMarketplaceRequest,
        user_identifier: &str,
    ) -> Result<MarketPlace, RepositoryError> {
        let query = "INSERT INTO marketplaces (identifier, name, description, user_identifier) VALUES ($1, $2, $3, $4) RETURNING *";

        let identifier = Ulid::new().to_string();
        let marketplace: MarketPlace = sqlx::query_as(query)
            .bind(identifier)
            .bind(&request.name)
            .bind(&request.description)
            .bind(user_identifier)
            .fetch_one(self.pool.as_ref())
            .await
            .map_err(|err| RepositoryError::from(err))?;

        Ok(marketplace)
    }

    async fn find_marketplace_by_identifier(
        &self,
        identifier: &str,
        user_identifier: &str,
    ) -> Result<MarketPlace, RepositoryError> {
        let query = "SELECT * FROM marketplaces WHERE identifier = $1 AND user_identifier = $2";

        let marketplace: MarketPlace = sqlx::query_as(query)
            .bind(identifier)
            .bind(user_identifier)
            .fetch_one(self.pool.as_ref())
            .await
            .map_err(|err| RepositoryError::from(err))?;
        Ok(marketplace)
    }

    async fn find_marketplace_by_name(
        &self,
        identifier: &str,
        user_identifier: &str,
    ) -> Result<MarketPlace, RepositoryError> {
        let query = "SELECT * FROM marketplaces WHERE name = $1 AND user_identifier = $2";

        let marketplace: MarketPlace = sqlx::query_as(query)
            .bind(identifier)
            .bind(user_identifier)
            .fetch_one(self.pool.as_ref())
            .await
            .map_err(|err| RepositoryError::from(err))?;

        Ok(marketplace)
    }

    //TODO: paginate
    async fn find_all_marketplaces(
        &self,
        user_identifier: &str,
    ) -> Result<Vec<MarketPlace>, RepositoryError> {
        let query = "SELECT * FROM marketplaces WHERE user_identifier = $1";

        let marketplaces: Vec<MarketPlace> = sqlx::query_as(query)
            .bind(user_identifier)
            .fetch_all(self.pool.as_ref())
            .await
            .map_err(|err| RepositoryError::from(err))?;

        Ok(marketplaces)
    }

    async fn update_marketplace_by_identifier(
        &self,
        identifier: &str,
        request: &CreateMarketplaceRequest,
        user_identifier: &str,
    ) -> Result<MarketPlace, RepositoryError> {
        let query = "UPDATE marketplaces SET name = $1, description = $2, updated_at = NOW() WHERE identifier = $3 AND user_identifier = $4 RETURNING *";

        let marketplace: MarketPlace = sqlx::query_as(query)
            .bind(&request.name)
            .bind(&request.description)
            .bind(identifier)
            .bind(user_identifier)
            .fetch_one(self.pool.as_ref())
            .await
            .map_err(|err| RepositoryError::from(err))?;

        Ok(marketplace)
    }

    async fn delete_marketplace_by_identifier(
        &self,
        identifier: &str,
        user_identifier: &str,
    ) -> Result<(), RepositoryError> {
        let query = "DELETE FROM marketplaces WHERE identifier = $1 AND user_identifier = $2";
        sqlx::query(query)
            .bind(identifier)
            .bind(user_identifier)
            .execute(self.pool.as_ref())
            .await
            .map_err(|err| RepositoryError::from(err))?;
        Ok(())
    }

    async fn marketplace_exists(
        &self,
        identifier: &str,
        user_identifier: &str,
    ) -> Result<bool, RepositoryError> {
        let query = "SELECT EXISTS(SELECT 1 FROM marketplaces WHERE identifier = $1 AND user_identifier = $2)";

        let exists: bool = sqlx::query_scalar(query)
            .bind(identifier)
            .bind(user_identifier)
            .fetch_one(self.pool.as_ref())
            .await
            .map_err(|err| RepositoryError::from(err))?;

        Ok(exists)
    }

    async fn count_marketplaces(&self, user_identifier: &str) -> Result<i64, RepositoryError> {
        let query = "SELECT COUNT(*) FROM marketplaces WHERE user_identifier = $1";

        let count: i64 = sqlx::query_scalar(query)
            .bind(user_identifier)
            .fetch_one(self.pool.as_ref())
            .await
            .map_err(|err| RepositoryError::from(err))?;

        Ok(count)
    }
}
