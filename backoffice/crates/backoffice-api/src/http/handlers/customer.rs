use std::sync::Arc;

use axum::{
    extract::{Path, State},
};

use backoffice_domain::errors::api_response::ApiResponse;
use backoffice_domain::errors::service_error::ServiceError;
use backoffice_domain::models::customers;
use backoffice_domain::services::customer::CustomerServiceExt;

use crate::http::dto::jwt::Claims;
use crate::state::AppState;

pub async fn find_all_customers(
    State(state): State<Arc<AppState>>,
    _claims: Claims,
) -> Result<ApiResponse<Vec<customers::Model>>, ServiceError> {
    let results = state.services.customer_service.find_all_customers().await?;

    Ok(ApiResponse::builder()
        .message("customers fetched successfully")
        .data(results)
        .build())
}

pub async fn find_customer_by_identifier(
    State(state): State<Arc<AppState>>,
    _claims: Claims,
    Path(identifier): axum::extract::Path<String>,
) -> Result<ApiResponse<customers::Model>, ServiceError> {
    let result = state
        .services
        .customer_service
        .find_customer_by_identifier(&identifier)
        .await?;

    Ok(ApiResponse::builder()
        .message("customer fetched successfully")
        .data(result)
        .build())
}

pub async fn count_customers(
    State(state): State<Arc<AppState>>,
    _claims: Claims,
) -> Result<ApiResponse<i64>, ServiceError> {
    let count = state.services.customer_service.count_customers().await?;

    Ok(ApiResponse::builder()
        .message("customers counted successfully")
        .data(count)
        .build())
}

pub async fn delete_customer_by_identifier(
    State(state): State<Arc<AppState>>,
    _claims: Claims,
    Path(identifier): Path<String>,
) -> Result<ApiResponse<()>, ServiceError> {
    state
        .services
        .customer_service
        .delete_customer_by_identifier(&identifier)
        .await?;

    Ok(ApiResponse::builder()
        .message("customer deleted successfully")
        .build())
}
