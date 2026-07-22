use std::sync::Arc;

use axum::{
    extract::{Path, State},
    http::StatusCode,
};

use backoffice_domain::errors::api_response::ApiResponse;
use backoffice_domain::errors::service_error::ServiceError;
use backoffice_domain::models::products::Model as Product;
use backoffice_domain::services::product::ProductServiceStateExt;

use crate::http::dto::jwt::Claims;
use crate::state::AppState;

pub async fn create_product(
    State(state): State<Arc<AppState>>,
    _claims: Claims,
    request: axum_typed_multipart::TypedMultipart<
        crate::http::extractors::products::CreateProductRequest,
    >,
) -> Result<ApiResponse<Product>, ServiceError> {
    let req = request.0;

    let picture_path = req
        .picture
        .contents
        .path()
        .to_string_lossy()
        .to_string();

    let product = state
        .services
        .product_service
        .add_product(&backoffice_domain::dto::SaveProductCommand {
            picture: picture_path,
            name: req.name,
            description: req.description,
            price: req.price,
            currency_identifier: req.currency_identifier,
        })
        .await?;

    Ok(ApiResponse::builder()
        .data(product)
        .message("product created successfully")
        .status_code(StatusCode::CREATED)
        .build())
}

pub async fn find_all_products(
    State(state): State<Arc<AppState>>,
) -> Result<ApiResponse<Vec<Product>>, ServiceError> {
    let products = state
        .services
        .product_service
        .fetch_all_products()
        .await?;

    Ok(ApiResponse::builder()
        .data(products)
        .message("products fetched successfully")
        .build())
}

pub async fn find_product(
    State(state): State<Arc<AppState>>,
    Path(product_identifier): Path<String>,
) -> Result<ApiResponse<Product>, ServiceError> {
    let product = state
        .services
        .product_service
        .fetch_product(&product_identifier)
        .await?;

    Ok(ApiResponse::builder()
        .data(product)
        .message("product fetched successfully")
        .build())
}
