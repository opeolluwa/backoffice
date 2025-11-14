use axum::extract::{Path, State};

use crate::{
    adapters::{
        dto::jwt::Claims,
        requests::{api_request::AuthenticatedRequest, marketplace::CreateMarketplaceRequest},
        response::api_response::ApiResponse,
    },
    entities::marketplace::MarketPlace,
    errors::service_error::ServiceError,
    services::marketplace_service::{MarketplaceService, MarketplaceServiceExt},
};

pub async fn create_marketplace(
    State(marketplace_service): State<MarketplaceService>,
    request: AuthenticatedRequest<CreateMarketplaceRequest>,
) -> Result<ApiResponse<MarketPlace>, ServiceError> {
    let marketplace = marketplace_service
        .create_marketplace(&request.data, &request.claims.identifier)
        .await?;

    Ok(ApiResponse::builder()
        .message("Marketplace created successfully")
        .data(marketplace)
        .build())
}

pub async fn find_marketplace_by_identifier(
    State(marketplace_service): State<MarketplaceService>,
    claims: Claims,
    Path(marketplace_identifier): axum::extract::Path<String>,
) -> Result<ApiResponse<MarketPlace>, ServiceError> {
    let marketplace = marketplace_service
        .find_marketplace_by_identifier(&marketplace_identifier, &claims.identifier)
        .await?;

    Ok(ApiResponse::builder()
        .message("Marketplace fetched successfully")
        .data(marketplace)
        .build())
}



pub async fn find_all_marketplaces(
    State(marketplace_service): State<MarketplaceService>,
    claims: Claims,
) -> Result<ApiResponse<Vec<MarketPlace>>, ServiceError> {
    let marketplaces = marketplace_service
        .find_all_marketplaces(&claims.identifier)
        .await?;

    Ok(ApiResponse::builder()
        .message("Marketplaces fetched successfully")
        .data(marketplaces)
        .build())
}


pub async fn count_marketplaces(
    State(marketplace_service): State<MarketplaceService>,
    claims: Claims,
) -> Result<ApiResponse<i64>, ServiceError> {
    let count = marketplace_service
        .count_marketplaces(&claims.identifier)
        .await?;

    Ok(ApiResponse::builder()
        .message("Marketplaces counted successfully")
        .data(count)
        .build())
}