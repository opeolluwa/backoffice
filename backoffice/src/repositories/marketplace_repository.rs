//! Marketplace repository module
//! The Marketplace contains different products, see it as a catalogue of items.
//!
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, PaginatorTrait, QueryFilter,
    Set,
};
use ulid::Ulid;

use crate::{
    adapters::requests::marketplace::CreateMarketplaceRequest,
    entities::marketplaces::{self, Entity as MarketplaceEntity},
    errors::database_error::DatabaseError,
    repositories::base::Repository,
};

#[derive(Debug, Clone)]
pub struct MarketplaceRepository {
    db: DatabaseConnection,
}

impl Repository for MarketplaceRepository {
    fn init(db: &DatabaseConnection) -> Self {
        Self { db: db.clone() }
    }
}

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

impl MarketplaceRepositoryExt for MarketplaceRepository {
    async fn create_marketplace(
        &self,
        request: &CreateMarketplaceRequest,
        user_identifier: &str,
    ) -> Result<marketplaces::Model, DatabaseError> {
        let model = marketplaces::ActiveModel {
            identifier: Set(Ulid::new().to_string()),
            name: Set(request.name.clone()),
            description: Set(request.description.clone()),
            slug: Set(request.slug.clone()),
            user_identifier: Set(Some(user_identifier.to_string())),
            ..Default::default()
        };
        let marketplace = model.insert(&self.db).await.map_err(DatabaseError::from)?;
        Ok(marketplace)
    }

    async fn find_marketplace_by_identifier(
        &self,
        identifier: &str,
        user_identifier: &str,
    ) -> Result<marketplaces::Model, DatabaseError> {
        MarketplaceEntity::find()
            .filter(marketplaces::Column::Identifier.eq(identifier))
            .filter(marketplaces::Column::UserIdentifier.eq(user_identifier))
            .one(&self.db)
            .await
            .map_err(DatabaseError::from)?
            .ok_or_else(|| DatabaseError::NotFound("marketplace not found".to_string()))
    }

    async fn find_marketplace_by_name(
        &self,
        name: &str,
        user_identifier: &str,
    ) -> Result<marketplaces::Model, DatabaseError> {
        MarketplaceEntity::find()
            .filter(marketplaces::Column::Name.eq(name))
            .filter(marketplaces::Column::UserIdentifier.eq(user_identifier))
            .one(&self.db)
            .await
            .map_err(DatabaseError::from)?
            .ok_or_else(|| DatabaseError::NotFound("marketplace not found".to_string()))
    }

    async fn find_all_marketplaces(
        &self,
        user_identifier: &str,
    ) -> Result<Vec<marketplaces::Model>, DatabaseError> {
        MarketplaceEntity::find()
            .filter(marketplaces::Column::UserIdentifier.eq(user_identifier))
            .all(&self.db)
            .await
            .map_err(DatabaseError::from)
    }

    async fn update_marketplace_by_identifier(
        &self,
        identifier: &str,
        request: &CreateMarketplaceRequest,
        user_identifier: &str,
    ) -> Result<marketplaces::Model, DatabaseError> {
        let marketplace = MarketplaceEntity::find()
            .filter(marketplaces::Column::Identifier.eq(identifier))
            .filter(marketplaces::Column::UserIdentifier.eq(user_identifier))
            .one(&self.db)
            .await
            .map_err(DatabaseError::from)?
            .ok_or_else(|| DatabaseError::NotFound("marketplace not found".to_string()))?;

        let mut active: marketplaces::ActiveModel = marketplace.into();
        active.name = Set(request.name.clone());
        active.description = Set(request.description.clone());
        active.updated_at = Set(Some(chrono::Utc::now().fixed_offset()));

        active.update(&self.db).await.map_err(DatabaseError::from)
    }

    async fn delete_marketplace_by_identifier(
        &self,
        identifier: &str,
        user_identifier: &str,
    ) -> Result<(), DatabaseError> {
        MarketplaceEntity::delete_many()
            .filter(marketplaces::Column::Identifier.eq(identifier))
            .filter(marketplaces::Column::UserIdentifier.eq(user_identifier))
            .exec(&self.db)
            .await
            .map_err(DatabaseError::from)?;
        Ok(())
    }

    async fn marketplace_exists(
        &self,
        identifier: &str,
        user_identifier: &str,
    ) -> Result<bool, DatabaseError> {
        let count = MarketplaceEntity::find()
            .filter(marketplaces::Column::Identifier.eq(identifier))
            .filter(marketplaces::Column::UserIdentifier.eq(user_identifier))
            .count(&self.db)
            .await
            .map_err(DatabaseError::from)?;
        Ok(count > 0)
    }

    async fn count_marketplaces(&self, user_identifier: &str) -> Result<i64, DatabaseError> {
        let count = MarketplaceEntity::find()
            .filter(marketplaces::Column::UserIdentifier.eq(user_identifier))
            .count(&self.db)
            .await
            .map_err(DatabaseError::from)?;
        Ok(count as i64)
    }
}
