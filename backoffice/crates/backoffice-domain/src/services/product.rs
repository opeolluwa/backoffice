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
    async fn add_product(&self, command: &SaveProductCommand) -> Result<Product, ServiceError>;

    async fn fetch_all_products(&self) -> Result<Vec<Product>, ServiceError>;

    async fn fetch_product(&self, product_identifier: &str) -> Result<Product, ServiceError>;

    async fn find_product_by_identifier(&self, identifier: &str) -> Result<Product, ServiceError>;
}

impl<R: ProductRepositoryExt + Send + Sync> ProductServiceStateExt for ProductService<R> {
    async fn add_product(&self, command: &SaveProductCommand) -> Result<Product, ServiceError> {
        let product = self.repo.create_product(command).await?;
        Ok(product)
    }

    async fn fetch_all_products(&self) -> Result<Vec<Product>, ServiceError> {
        let products = self.repo.find_all_products().await?;
        Ok(products)
    }

    async fn fetch_product(&self, product_identifier: &str) -> Result<Product, ServiceError> {
        let product = self.repo.retrieve_product(product_identifier).await?;
        Ok(product)
    }

    async fn find_product_by_identifier(&self, identifier: &str) -> Result<Product, ServiceError> {
        let product = self.repo.find_product_by_identifier(identifier).await?;
        Ok(product)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ports::product_repository::MockProductRepositoryExt;
    use sea_orm::sqlx::types::chrono::Utc;

    fn test_product() -> crate::models::products::Model {
        crate::models::products::Model {
            identifier: "prod-001".to_string(),
            name: "Widget".to_string(),
            picture: Some("https://example.com/widget.jpg".to_string()),
            price: 23,
            description: "A fine widget".to_string(),
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
            .returning(move |_| Ok(product.clone()));
        let service = ProductService::new(repo);

        let result = service.fetch_product("prod-001").await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap().name, "Widget");
    }

    #[tokio::test]
    async fn fetch_product_not_found() {
        let mut repo = MockProductRepositoryExt::new();
        repo.expect_retrieve_product().returning(|_| {
            Err(crate::errors::database_error::DatabaseError::NotFound(
                "not found".into(),
            ))
        });
        let service = ProductService::new(repo);

        let result = service.fetch_product("nonexistent").await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn fetch_all_products() {
        let mut repo = MockProductRepositoryExt::new();
        repo.expect_find_all_products()
            .returning(|| Ok(vec![test_product(), test_product()]));
        let service = ProductService::new(repo);

        let result = service.fetch_all_products().await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap().len(), 2);
    }

    #[tokio::test]
    async fn add_product_success() {
        let mut repo = MockProductRepositoryExt::new();
        let product = test_product();
        repo.expect_create_product()
            .returning(move |_| Ok(product.clone()));
        let service = ProductService::new(repo);

        let cmd = SaveProductCommand {
            picture: Some("https://example.com/widget.jpg".to_string()),
            name: "Widget".to_string(),
            description: "A fine widget".to_string(),
            price: 2999,
            currency_identifier: "NG".to_string(),
        };
        let result = service.add_product(&cmd).await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap().name, "Widget");
    }
}
