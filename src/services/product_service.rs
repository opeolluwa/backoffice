use axum_typed_multipart::TypedMultipart;
use backoffice_imagekit::ImagekitClient;
use backoffice_utils::extract_env;
use sqlx::PgPool;

use crate::{
    adapters::requests::products::{CreateProductRequest, SaveProductRequest},
    config::app_config::AppConfig,
    entities::products::Product,
    errors::{app_error::AppError, service_error::ServiceError},
    fs::filesystem::AppFileSystem,
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

    async fn fetch_product(
        &self,
        product_identifier: &str,
        user_identifier: &str,
    ) -> Result<Product, ServiceError>;
}

impl ProductServiceStateExt for ProductService {
    async fn add_product(
        &self,
        TypedMultipart(CreateProductRequest {
            picture,
            price,
            name,
            description,
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
            .upload_file(file.file_path, &file.file_name)
            .await?;

        let save_product = SaveProductRequest {
            picture: upload_response.url,
            name,
            description,
            price,
        };

        let product = self
            .product_repository
            .create_product(&save_product, user_identifier, marketplace_identifier)
            .await?;

        // delete file
        // file_system.delete_file_if_exists(file.file_path.to_str().unwrap());
        Ok(product)
    }

    async fn fetch_product(
        &self,
        product_identifier: &str,
        user_identifier: &str,
    ) -> Result<Product, ServiceError> {
        let product = self
            .product_repository
            .retrieve_product(user_identifier, product_identifier)
            .await?;

        Ok(product)
    }
}
