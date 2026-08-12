use std::sync::Arc;

use axum::extract::State;
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::errors::api_response::ApiResponse;
use crate::errors::service_error::ServiceError;
use crate::models::app_config;
use crate::services::app_config_services::AppConfigServiceExt;

use crate::http::dto::api_request::AuthenticatedRequest;
use crate::state::AppState;

#[derive(Debug, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct UpdateAppConfigRequest {
    pub app_name: Option<Option<String>>,
    pub support_email: Option<Option<String>>,
    pub default_currency: Option<Option<String>>,
    pub default_language: Option<Option<String>>,
    pub maintenance_mode: Option<bool>,
    pub logo_url: Option<Option<String>>,
}

pub async fn fetch_app_config(
    State(state): State<Arc<AppState>>,
) -> Result<ApiResponse<Option<app_config::Model>>, ServiceError> {
    let config = state.services.app_config_service.get_app_config().await?;
    Ok(ApiResponse::builder()
        .message("App config fetched successfully")
        .data(config)
        .build())
}

pub async fn update_app_config(
    State(state): State<Arc<AppState>>,
    AuthenticatedRequest { data, .. }: AuthenticatedRequest<UpdateAppConfigRequest>,
) -> Result<ApiResponse<app_config::Model>, ServiceError> {
    let config = state
        .services
        .app_config_service
        .update_app_config(
            data.app_name,
            data.support_email,
            data.default_currency,
            data.default_language,
            data.maintenance_mode,
            data.logo_url,
        )
        .await?;

    Ok(ApiResponse::builder()
        .message("App config updated successfully")
        .data(config)
        .build())
}
