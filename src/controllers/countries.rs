use axum::extract::{Path, State};

use crate::{
    adapters::response::api_response::ApiResponse, entities::country::Country,
    errors::service_error::ServiceError, services::country_service::CountryService,
    services::country_service::CountryServiceExt,
};

pub async fn fetch_all_countries(
    State(country_service): State<CountryService>,
) -> Result<ApiResponse<Vec<Country>>, ServiceError> {
    let countries = country_service.get_all_countries().await?;

    Ok(ApiResponse::builder()
        .message("Countries fetched successfully")
        .data(countries)
        .build())
}

pub async fn fetch_country_by_identifier(
    State(country_service): State<CountryService>,
    Path(identifier): Path<String>,
) -> Result<ApiResponse<Option<Country>>, ServiceError> {
    let country = country_service
        .get_country_by_identifier(&identifier)
        .await?;

    Ok(ApiResponse::builder()
        .message("Country fetched successfully")
        .data(country)
        .build())
}
