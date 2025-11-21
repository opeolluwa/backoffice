use axum_typed_multipart::TypedMultipart;
use sqlx::PgPool;

use crate::{
    adapters::requests::products::{CreateProductRequest, SaveProductRequest},
    entities::products::Product,
    errors::service_error::ServiceError,
    repositories::{
        base::Repository,
        product_repository::{ProductRepository, ProductRepositoryExt},
    },
};

#[derive(Clone)]
pub struct ProductService {
    product_repository: ProductRepository,
}

impl ProductService {
    pub fn init(pool: &PgPool) -> Self {
        Self {
            product_repository: ProductRepository::init(pool),
        }
    }
}

pub(crate) trait ProductServiceStateExt {
    async fn add_product(
        &self,
        request: TypedMultipart<CreateProductRequest>,
        user_identifier: &str,
        marketplace_identifier: &str,
    ) -> Result<Product, ServiceError>;

    async fn fetch_product(&self, user_identifier: &str) -> Result<Product, ServiceError>;
}

impl ProductServiceStateExt for ProductService {
    async fn add_product(
        &self,
        request: TypedMultipart<CreateProductRequest>,
        user_identifier: &str,
        marketplace_identifier: &str,
    ) -> Result<Product, ServiceError> {
        let save_product = SaveProductRequest {
            picture: todo!(),
            name: todo!(),
            description: todo!(),
            price: todo!(),
        };

        let product = self
            .product_repository
            .create_product(&save_product, user_identifier, marketplace_identifier)
            .await?;

        Ok(product)
    }

    async fn fetch_product(&self, identifier: &str) -> Result<Product, ServiceError> {
        let product = self.product_repository.retrieve_product(identifier).await?;

        Ok(product)
    }
}
