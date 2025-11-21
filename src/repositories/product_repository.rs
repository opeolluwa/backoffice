use std::sync::Arc;

use sqlx::PgPool;

use crate::{
    adapters::requests::products::SaveProductRequest,
    entities::{marketplace::MarketplaceWithProducts, products::Product},
    errors::repository_error::RepositoryError,
    repositories::base::Repository,
};
 use sqlx::types::Json;

#[derive(Clone)]
pub struct ProductRepository {
    pool: Arc<PgPool>,
}

impl Repository for ProductRepository {
    fn init(pool: &PgPool) -> Self {
        Self {
            pool: Arc::new(pool.clone()),
        }
    }
}

pub(crate) trait ProductRepositoryExt {
    async fn create_product(
        &self,
        request: &SaveProductRequest,
        user_identifier: &str,
        marketplace_identifier: &str,
    ) -> Result<Product, RepositoryError>;

    async fn retrieve_product(
        &self,
        identifier: &str,
        user_identifier: &str,
    ) -> Result<Product, RepositoryError>;

    async fn fetch_marketplace_products(
        &self,
        marketplace_identifier: &str,
        user_identifier: &str,
    ) -> Result<MarketplaceWithProducts, RepositoryError>;
}

impl ProductRepositoryExt for ProductRepository {
    async fn create_product(
        &self,
        request: &SaveProductRequest,
        user_identifier: &str,
        marketplace_identifier: &str,
    ) -> Result<Product, RepositoryError> {
        let identifier = ulid::Ulid::new().to_string();
        let name = &request.name;
        let picture = &request.picture;
        let price = rust_decimal::Decimal::new(request.price, 2);
        let description = &request.description;
        let created_by_identifier = user_identifier;
        let marketplace_identifier = marketplace_identifier;

        let product = sqlx::query_as!(
            Product,
            r#"
            INSERT INTO products (
                identifier,
                name,
                picture,
                price,
                description,
                created_by_identifier,
                marketplace_identifier
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            RETURNING
                identifier,
                name,
                picture,
                price,
                description,
                created_by_identifier,
                marketplace_identifier,
                created_at,
                updated_at
            "#,
            identifier,
            name,
            picture,
            price,
            description,
            created_by_identifier,
            marketplace_identifier
        )
        .fetch_one(&*self.pool)
        .await
        .map_err(RepositoryError::from)?;

        Ok(product)
    }

    async fn retrieve_product(
        &self,
        identifier: &str,
        user_identifier: &str,
    ) -> Result<Product, RepositoryError> {
        let product = sqlx::query_as!(
            Product,
            r#"
            SELECT
                identifier,
                name,
                picture,
                price,
                description,
                created_by_identifier,
                marketplace_identifier,
                created_at,
                updated_at
            FROM products
            WHERE identifier = $1
            AND created_by_identifier = $2
            "#,
            identifier,
            user_identifier
        )
        .fetch_one(&*self.pool)
        .await
        .map_err(RepositoryError::from)?;

        Ok(product)
    }

    async fn fetch_marketplace_products(
        &self,
        marketplace_identifier: &str,
        user_identifier: &str,
    ) -> Result<MarketplaceWithProducts, RepositoryError> {
      let marketplace = sqlx::query_as!(
    MarketplaceWithProducts,
    r#"
        SELECT
            m.identifier,
            m.user_identifier,
            m.created_at,
            m.updated_at,
            m.name,
            m.description,
            COALESCE(
                json_agg(
                    jsonb_build_object(
                        'identifier', p.identifier,
                        'name', p.name,
                        'price', p.price,
                        'description', p.description,
                        'picture', p.picture,
                        'created_by_identifier', p.created_by_identifier,
                        'marketplace_identifier', p.marketplace_identifier,
                        'created_at', p.created_at,
                        'updated_at', p.updated_at
                    )
                ) FILTER (WHERE p.identifier IS NOT NULL),
                '[]'
            ) AS "products!: Json<Vec<Product>>"
        FROM marketplaces m
        LEFT JOIN products p ON p.marketplace_identifier = m.identifier
        WHERE m.identifier = $1
          AND m.user_identifier = $2
        GROUP BY m.identifier
    "#,
    marketplace_identifier,
    user_identifier
)
.fetch_one(&*self.pool)
.await?;


    Ok(marketplace)
    }
}
