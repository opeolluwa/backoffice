use axum::extract::{Path, State};
use axum_typed_multipart::TypedMultipart;

use crate::{
    adapters::{
        dto::jwt::Claims,
        requests::{api_request::AuthenticatedRequest, products::CreateProductRequest},
        response::api_response::ApiResponse,
    },
    entities::products::Product,
    errors::service_error::ServiceError,
    services::product_service::{ProductService, ProductServiceStateExt},
};

pub async fn add_product_to_marketplace(
    State(product_service): State<ProductService>,
    claims: Claims,
    Path(marketplace_identifier): Path<String>,
    request: TypedMultipart<CreateProductRequest>,
) -> Result<ApiResponse<Product>, ServiceError> {
    let product = product_service
        .add_product(request, &claims.identifier, &marketplace_identifier)
        .await?;

    todo!()
}

pub async fn retrieve_product_from_marketplace(
    State(product_service): State<ProductService>,
    Path(marketplace_identifier): Path<String>,
    Path(product_identifier): Path<String>,
) -> Result<ApiResponse<Product>, ServiceError> {
    // let product = product_service
    //     .add_product(request, user_identifier, marketplace_identifier, picture)
    //     .await?;

    todo!()
}
