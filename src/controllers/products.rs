use axum::extract::{Path, State};

use crate::{
    adapters::{
        requests::{api_request::AuthenticatedRequest, products::CreateProductRequest},
        response::api_response::ApiResponse,
    },
    entities::products::Product,
    errors::service_error::ServiceError,
    services::product_service::ProductService,
    services::product_service::ProductServiceStateExt,
};

pub async fn add_product_to_marketplace(
    State(product_service): State<ProductService>,
    Path(marketplace_identifier): Path<String>,
    request: AuthenticatedRequest<CreateProductRequest>,
) -> Result<ApiResponse<Product>, ServiceError> {
    // let product = product_service
    //     .add_product(request, user_identifier, marketplace_identifier, picture)
    //     .await?;

    todo!()
}



pub async fn retrieve_product_from_marketplace(
    State(product_service): State<ProductService>,
    Path(marketplace_identifier): Path<String>,
    request: AuthenticatedRequest<CreateProductRequest>,
) -> Result<ApiResponse<Product>, ServiceError> {
    // let product = product_service
    //     .add_product(request, user_identifier, marketplace_identifier, picture)
    //     .await?;

    todo!()
}