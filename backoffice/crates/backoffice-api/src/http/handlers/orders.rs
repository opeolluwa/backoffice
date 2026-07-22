use std::sync::Arc;

use axum::{
    extract::{Path, State},
    http::StatusCode,
};

use backoffice_domain::dto::PlaceOrderCommand;
use backoffice_domain::errors::api_response::ApiResponse;
use backoffice_domain::errors::service_error::ServiceError;
use backoffice_domain::models::orders;
use backoffice_domain::services::orders::OrderServiceExt;

use crate::http::dto::{api_request::AuthenticatedRequest, jwt::Claims};
use crate::http::extractors::orders::CreateOrdersRequest;
use crate::state::AppState;

fn to_command(req: &CreateOrdersRequest) -> PlaceOrderCommand {
    PlaceOrderCommand {
        items: vec![backoffice_domain::dto::PlaceOrderItem {
            product_identifier: req.product_identifier.clone(),
            quantity: req.quantity,
        }],
    }
}

pub async fn create_orders(
    State(state): State<Arc<AppState>>,
    request: AuthenticatedRequest<CreateOrdersRequest>,
) -> Result<
    ApiResponse<Vec<(orders::Model, backoffice_domain::models::products::Model)>>,
    ServiceError,
> {
    let command = to_command(&request.data);
    let result = state.services.orders_service.place_orders(&command).await?;

    Ok(ApiResponse::builder()
        .message("Orders created successfully")
        .status_code(StatusCode::CREATED)
        .data(result)
        .build())
}

pub async fn find_orders_by_identifier(
    State(state): State<Arc<AppState>>,
    _claims: Claims,
    Path(identifier): axum::extract::Path<String>,
) -> Result<ApiResponse<orders::Model>, ServiceError> {
    let result = state
        .services
        .orders_service
        .find_orders_by_identifier(&identifier)
        .await?;

    Ok(ApiResponse::builder()
        .message("Orders fetched successfully")
        .data(result)
        .build())
}

pub async fn find_all_orders(
    State(state): State<Arc<AppState>>,
    _claims: Claims,
) -> Result<ApiResponse<Vec<orders::Model>>, ServiceError> {
    let results = state.services.orders_service.find_all_orders().await?;

    Ok(ApiResponse::builder()
        .message("orders fetched successfully")
        .data(results)
        .build())
}

pub async fn count_orders(
    State(state): State<Arc<AppState>>,
    _claims: Claims,
) -> Result<ApiResponse<i64>, ServiceError> {
    let count = state.services.orders_service.count_orders().await?;

    Ok(ApiResponse::builder()
        .message("orders counted successfully")
        .data(count)
        .build())
}

pub async fn update_orders_by_identifier(
    State(state): State<Arc<AppState>>,
    Path(identifier): Path<String>,
    AuthenticatedRequest { data, .. }: AuthenticatedRequest<CreateOrdersRequest>,
) -> Result<ApiResponse<orders::Model>, ServiceError> {
    let command = to_command(&data);
    let result = state
        .services
        .orders_service
        .update_orders_by_identifier(&identifier, &command)
        .await?;

    Ok(ApiResponse::builder()
        .message("Orders updated successfully")
        .data(result)
        .build())
}

pub async fn delete_orders_by_identifier(
    State(state): State<Arc<AppState>>,
    _claims: Claims,
    Path(identifier): Path<String>,
) -> Result<ApiResponse<()>, ServiceError> {
    state
        .services
        .orders_service
        .delete_orders_by_identifier(&identifier)
        .await?;

    Ok(ApiResponse::builder()
        .message("Orders deleted successfully")
        .build())
}
