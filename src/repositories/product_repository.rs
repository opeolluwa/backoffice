use std::sync::Arc;

use sqlx::PgPool;

use crate::{
    adapters::requests::products::SaveProductRequest, entities::products::Product,
    errors::repository_error::RepositoryError, repositories::base::Repository,
};

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

    async fn retrieve_product(&self, identifier: &str) -> Result<Product, RepositoryError>;
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

    async fn retrieve_product(&self, identifier: &str) -> Result<Product, RepositoryError> {
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
            "#,
            identifier,
        )
        .fetch_one(&*self.pool)
        .await
        .map_err(RepositoryError::from)?;

        Ok(product)
    }
}
