use std::sync::Arc;

use axum::{
    extract::{Path, State},
    http::StatusCode,
};

use crate::http::dto::jwt::Claims;
use backoffice_domain::errors::api_response::ApiResponse;
use crate::state::AppState;
use backoffice_domain::models::products::Model as Product;
use backoffice_domain::services::product::ProductServiceStateExt;
use backoffice_domain::errors::service_error::ServiceError;

pub async fn add_product_to_marketplace(
    State(state): State<Arc<AppState>>,
    claims: Claims,
    Path(marketplace_identifier): Path<String>,
    _request: axum_typed_multipart::TypedMultipart<crate::http::extractors::products::CreateProductRequest>,
) -> Result<ApiResponse<Product>, ServiceError> {
    let product = state
        .services
        .product_service
        .add_product(
            &backoffice_domain::dto::SaveProductCommand {
                picture: String::new(),
                name: String::new(),
                description: String::new(),
                price: 0,
                currency_identifier: String::new(),
            },
            &claims.identifier,
            &marketplace_identifier,
        )
        .await?;

    Ok(ApiResponse::builder()
        .data(product)
        .message("product created successfully")
        .status_code(StatusCode::CREATED)
        .build())
}

pub async fn retrieve_product_from_marketplace(
    State(state): State<Arc<AppState>>,
    claims: Claims,
    Path(product_identifier): Path<String>,
) -> Result<ApiResponse<Product>, ServiceError> {
    let product = state
        .services
        .product_service
        .fetch_product(&product_identifier, &claims.identifier)
        .await?;

    Ok(ApiResponse::builder()
        .data(product)
        .message("marketplace product")
        .build())
}
