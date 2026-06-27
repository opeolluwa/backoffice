use std::sync::Arc;

use axum::{
    extract::{Path, State},
    http::StatusCode,
};

use crate::{
    api::http::dto::{api_request::AuthenticatedRequest, api_response::ApiResponse, jwt::Claims},
    api::http::extractors::marketplace::CreateMarketplaceRequest,
    api::state::AppState,
    domain::services::marketplace::MarketplaceServiceExt,
    entities::marketplaces,
    errors::service_error::ServiceError,
};

pub async fn create_marketplace(
    State(state): State<Arc<AppState>>,
    request: AuthenticatedRequest<CreateMarketplaceRequest>,
) -> Result<ApiResponse<marketplaces::Model>, ServiceError> {
    let marketplace = state
        .services
        .marketplace_service
        .create_marketplace(&request.data, &request.claims.identifier)
        .await?;

    Ok(ApiResponse::builder()
        .message("Marketplace created successfully")
        .status_code(StatusCode::CREATED)
        .data(marketplace)
        .build())
}

pub async fn find_marketplace_by_identifier(
    State(state): State<Arc<AppState>>,
    claims: Claims,
    Path(marketplace_identifier): axum::extract::Path<String>,
) -> Result<ApiResponse<marketplaces::Model>, ServiceError> {
    let marketplace = state
        .services
        .marketplace_service
        .find_marketplace_by_identifier(&marketplace_identifier, &claims.identifier)
        .await?;

    Ok(ApiResponse::builder()
        .message("Marketplace fetched successfully")
        .data(marketplace)
        .build())
}

pub async fn find_all_marketplaces(
    State(state): State<Arc<AppState>>,
    claims: Claims,
) -> Result<ApiResponse<Vec<marketplaces::Model>>, ServiceError> {
    let marketplaces = state
        .services
        .marketplace_service
        .find_all_marketplaces(&claims.identifier)
        .await?;

    Ok(ApiResponse::builder()
        .message("Marketplaces fetched successfully")
        .data(marketplaces)
        .build())
}

pub async fn count_marketplaces(
    State(state): State<Arc<AppState>>,
    claims: Claims,
) -> Result<ApiResponse<i64>, ServiceError> {
    let count = state
        .services
        .marketplace_service
        .count_marketplaces(&claims.identifier)
        .await?;

    Ok(ApiResponse::builder()
        .message("Marketplaces counted successfully")
        .data(count)
        .build())
}

pub async fn update_marketplace_by_identifier(
    State(state): State<Arc<AppState>>,
    Path(identifier): Path<String>,
    AuthenticatedRequest { data, claims }: AuthenticatedRequest<CreateMarketplaceRequest>,
) -> Result<ApiResponse<marketplaces::Model>, ServiceError> {
    let updated_marketplace = state
        .services
        .marketplace_service
        .update_marketplace_by_identifier(&identifier, &data, &claims.identifier)
        .await?;

    Ok(ApiResponse::builder()
        .message("Marketplaces updated successfully")
        .data(updated_marketplace)
        .build())
}

pub async fn delete_marketplace_by_identifier(
    State(state): State<Arc<AppState>>,
    claims: Claims,
    Path(identifier): Path<String>,
) -> Result<ApiResponse<()>, ServiceError> {
    state
        .services
        .marketplace_service
        .delete_marketplace_by_identifier(&identifier, &claims.identifier)
        .await?;

    Ok(ApiResponse::builder()
        .message("Marketplace deleted successfully")
        .build())
}
