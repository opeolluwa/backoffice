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
    _request: axum_typed_multipart::TypedMultipart<
        crate::http::extractors::products::CreateProductRequest,
    >,
) -> Result<ApiResponse<Product>, ServiceError> {
    let product = state
        .services
        .product_service
        .add_product(&backoffice_domain::dto::SaveProductCommand {
            picture: String::new(),
            name: String::new(),
            description: String::new(),
            price: 0,
            currency_identifier: String::new(),
        })
        .await?;

    Ok(ApiResponse::builder()
        .data(product)
        .message("product created successfully")
        .status_code(StatusCode::CREATED)
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
