use crate::{
    dto::SaveProductCommand,
    models::products::Model as Product,
    ports::product_repository::ProductRepositoryExt,
    errors::service_error::ServiceError,
};

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
