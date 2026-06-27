use std::sync::Arc;

use axum::extract::{Path, State};

use crate::{
    api::{http::dto::api_response::ApiResponse, state::AppState},
    domain::services::country::CountryServiceExt,
    entities::countries,
    errors::service_error::ServiceError,
};

pub async fn fetch_all_countries(
    State(state): State<Arc<AppState>>,
) -> Result<ApiResponse<Vec<countries::Model>>, ServiceError> {
    let countries = state.services.country_service.get_all_countries().await?;

    Ok(ApiResponse::builder()
        .message("Countries fetched successfully")
        .data(countries)
        .build())
}

pub async fn fetch_country_by_identifier(
    State(state): State<Arc<AppState>>,
    Path(identifier): Path<String>,
) -> Result<ApiResponse<Option<countries::Model>>, ServiceError> {
    let country = state
        .services
        .country_service
        .get_country_by_identifier(&identifier)
        .await?;

    Ok(ApiResponse::builder()
        .message("Country fetched successfully")
        .data(country)
        .build())
}
