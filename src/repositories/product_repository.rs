use rust_decimal::Decimal;
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set};
use ulid::Ulid;

use crate::{
    adapters::requests::products::SaveProductRequest,
    entities::{
        countries,
        marketplaces::{self, Entity as MarketplaceEntity},
        products::{self, Entity as ProductEntity},
    },
    errors::database_error::DatabaseError,
    repositories::base::Repository,
};

#[derive(Debug, Clone)]
pub struct ProductRepository {
    db: DatabaseConnection,
}

impl Repository for ProductRepository {
    fn init(db: &DatabaseConnection) -> Self {
        Self { db: db.clone() }
    }
}

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

    // async fn fetch_marketplace_products(
    //     &self,
    //     marketplace_identifier: &str,
    //     user_identifier: &str,
    // ) -> Result<MarketplaceWithProducts, DatabaseError>;
}

impl ProductRepositoryExt for ProductRepository {
    async fn create_product(
        &self,
        request: &SaveProductRequest,
        user_identifier: &str,
        marketplace_identifier: &str,
    ) -> Result<products::Model, DatabaseError> {
        let model = products::ActiveModel {
            identifier: Set(Ulid::new().to_string()),
            name: Set(request.name.clone()),
            picture: Set(Some(request.picture.clone())),
            price: Set(Decimal::from(request.price)),
            description: Set(request.description.clone()),
            created_by_identifier: Set(Some(user_identifier.to_string())),
            marketplace_identifier: Set(Some(marketplace_identifier.to_string())),
            currency_identifier: Set(Some(request.currency_identifier.clone())),
            ..Default::default()
        };
        model.insert(&self.db).await.map_err(DatabaseError::from)
    }

    async fn retrieve_product(
        &self,
        identifier: &str,
        user_identifier: &str,
    ) -> Result<products::Model, DatabaseError> {
        ProductEntity::find()
            .filter(products::Column::Identifier.eq(identifier))
            .filter(products::Column::CreatedByIdentifier.eq(user_identifier))
            .one(&self.db)
            .await
            .map_err(DatabaseError::from)?
            .ok_or_else(|| DatabaseError::NotFound("product not found".to_string()))
    }

    // async fn fetch_marketplace_products(
    //     &self,
    //     marketplace_identifier: &str,
    //     user_identifier: &str,
    // ) -> Result<MarketplaceWithProducts, DatabaseError> {
    //     let marketplace = MarketplaceEntity::find()
    //         .filter(marketplaces::Column::Identifier.eq(marketplace_identifier))
    //         .filter(marketplaces::Column::UserIdentifier.eq(user_identifier))
    //         .one(&self.db)
    //         .await
    //         .map_err(DatabaseError::from)?
    //         .ok_or_else(|| DatabaseError::NotFound("marketplace not found".to_string()))?;

    //     let products_with_currencies: Vec<(products::Model, Option<countries::Model>)> =
    //         ProductEntity::find()
    //             .filter(products::Column::MarketplaceIdentifier.eq(marketplace_identifier))
    //             .find_also_related(countries::Entity)
    //             .all(&self.db)
    //             .await
    //             .map_err(DatabaseError::from)?;

    //     let products = products_with_currencies
    //         .into_iter()
    //         .map(|(product, country)| ProductWithCurrency::from_models(product, country))
    //         .collect();

    //     Ok(MarketplaceWithProducts::from_marketplace(
    //         marketplace,
    //         products,
    //     ))
    // }
}
