use std::sync::Arc;

use axum::{
    extract::{Path, State},
    http::StatusCode,
};
use axum_typed_multipart::TypedMultipart;

use crate::{
    api::http::dto::{api_response::ApiResponse, jwt::Claims},
    api::http::extractors::products::CreateProductRequest,
    api::state::AppState,
    domain::services::product::ProductServiceStateExt,
    entities::products::Model as Product,
    errors::service_error::ServiceError,
};

pub async fn add_product_to_marketplace(
    State(state): State<Arc<AppState>>,
    claims: Claims,
    Path(marketplace_identifier): Path<String>,
    request: TypedMultipart<CreateProductRequest>,
) -> Result<ApiResponse<Product>, ServiceError> {
    let product = state
        .services
        .product_service
        .add_product(request, &claims.identifier, &marketplace_identifier)
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
