use axum_typed_multipart::TypedMultipart;

use backoffice_imagekit::ImagekitClient;
use backoffice_utils::extract_env;

use crate::{
    adapters::requests::products::{CreateProductRequest, SaveProductRequest},
    config::app_config::AppConfig,
    domain::ports::product_repository::ProductRepositoryExt,
    entities::products::Model as Product,
    errors::{app_error::AppError, service_error::ServiceError},
    infrastructure::fs::AppFileSystem,
};

#[derive(Clone)]
pub struct ProductService<R: ProductRepositoryExt> {
    repo: R,
}

impl<R: ProductRepositoryExt + Clone> ProductService<R> {
    pub fn new(repo: R) -> Self {
        Self { repo }
    }
}

pub(crate) trait ProductServiceStateExt {
    async fn add_product(
        &self,
        request: TypedMultipart<CreateProductRequest>,
        user_identifier: &str,
        marketplace_identifier: &str,
    ) -> Result<Product, ServiceError>;

    async fn fetch_product(
        &self,
        product_identifier: &str,
        user_identifier: &str,
    ) -> Result<Product, ServiceError>;
}

impl<R: ProductRepositoryExt + Clone + Send + Sync> ProductServiceStateExt for ProductService<R> {
    async fn add_product(
        &self,
        TypedMultipart(CreateProductRequest {
            picture,
            price,
            name,
            description,
            currency_identifier,
        }): TypedMultipart<CreateProductRequest>,
        user_identifier: &str,
        marketplace_identifier: &str,
    ) -> Result<Product, ServiceError> {
        let private_key: String = extract_env("IMAGEKIT_PRIVATE_KEY")
            .map_err(|err| AppError::OperationFailed(err.to_string()))?;
        let public_key: String = extract_env("IMAGEKIT_PUBLIC_KEY")
            .map_err(|err| AppError::OperationFailed(err.to_string()))?;

        let imagekit_client =
            ImagekitClient::new(&public_key, &private_key).map_err(ServiceError::from)?;

        let config = AppConfig::from_env()?;
        let file_system = AppFileSystem::new(&config)?;

        let file = file_system.save_file_to_disk(picture)?;

        let upload_response = imagekit_client
            .upload_file(file.file_path.clone(), &file.file_name)
            .await?;

        let save_product = SaveProductRequest {
            picture: upload_response.url,
            name,
            description,
            price,
            currency_identifier,
        };

        let product = self
            .repo
            .create_product(&save_product, user_identifier, marketplace_identifier)
            .await?;

        file_system.delete_file_if_exists(file.file_path.to_str().unwrap())?;
        Ok(product)
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
