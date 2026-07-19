use crate::{
    dto::SaveProductCommand, errors::service_error::ServiceError,
    models::products::Model as Product, ports::product_repository::ProductRepositoryExt,
};

#[derive(Clone)]
pub struct ProductService<R: ProductRepositoryExt> {
    repo: R,
}

impl<R: ProductRepositoryExt> ProductService<R> {
    pub fn new(repo: R) -> Self {
        Self { repo }
    }
}

pub trait ProductServiceStateExt {
    async fn add_product(
        &self,
        command: &SaveProductCommand,
        user_identifier: &str,
        marketplace_identifier: &str,
    ) -> Result<Product, ServiceError>;

    async fn fetch_product(
        &self,
        product_identifier: &str,
        user_identifier: &str,
    ) -> Result<Product, ServiceError>;
}

impl<R: ProductRepositoryExt + Send + Sync> ProductServiceStateExt for ProductService<R> {
    async fn add_product(
        &self,
        _command: &SaveProductCommand,
        _user_identifier: &str,
        _marketplace_identifier: &str,
    ) -> Result<Product, ServiceError> {
        todo!("implement file upload")
    }

    async fn fetch_product(
        &self,
        product_identifier: &str,
        user_identifier: &str,
    ) -> Result<Product, ServiceError> {
        let product = self
            .repo
            .retrieve_product(user_identifier, product_identifier)
            .await?;

        Ok(product)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ports::product_repository::MockProductRepositoryExt;
    use rust_decimal::dec;
    use sea_orm::sqlx::types::chrono::Utc;

    fn test_product() -> crate::models::products::Model {
        crate::models::products::Model {
            identifier: "prod-001".to_string(),
            name: "Widget".to_string(),
            picture: Some("https://example.com/widget.jpg".to_string()),
            price: dec!(29.99),
            description: "A fine widget".to_string(),
            created_by_identifier: Some("user-001".to_string()),
            marketplace_identifier: Some("mp-001".to_string()),
            created_at: Utc::now().naive_utc().and_utc().into(),
            updated_at: None,
            currency_identifier: Some("NG".to_string()),
        }
    }

    #[tokio::test]
    async fn fetch_product_found() {
        let mut repo = MockProductRepositoryExt::new();
        let product = test_product();
        repo.expect_retrieve_product()
            .returning(move |_, _| Ok(product.clone()));
        let service = ProductService::new(repo);

        let result = service.fetch_product("prod-001", "user-001").await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap().name, "Widget");
    }

    #[tokio::test]
    async fn fetch_product_not_found() {
        let mut repo = MockProductRepositoryExt::new();
        repo.expect_retrieve_product().returning(|_, _| {
            Err(crate::errors::database_error::DatabaseError::NotFound(
                "not found".into(),
            ))
        });
        let service = ProductService::new(repo);

        let result = service.fetch_product("nonexistent", "user-001").await;
        assert!(result.is_err());
    }
}
