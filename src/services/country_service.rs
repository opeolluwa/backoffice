use sqlx::PgPool;

use crate::{
    entities::country::Country, errors::service_error::ServiceError,
    repositories::base::Repository, repositories::country_repository::CountryRepository,
    repositories::country_repository::CountryRepositoryExt,
};

#[derive(Clone)]
pub struct CountryService {
    country_repository: CountryRepository,
}

impl CountryService {
    pub fn init(pool: &PgPool) -> Self {
        Self {
            country_repository: CountryRepository::init(pool),
        }
    }
}

pub(crate) trait CountryServiceExt {
    async fn get_all_countries(&self) -> Result<Vec<Country>, ServiceError>;

    async fn get_country_by_identifier(
        &self,
        identifier: &str,
    ) -> Result<Option<Country>, ServiceError>;
}

impl CountryServiceExt for CountryService {
    async fn get_all_countries(&self) -> Result<Vec<Country>, ServiceError> {
        let countries = self
            .country_repository
            .fetch_all_countries()
            .await
            .map_err(ServiceError::from)?;

        Ok(countries)
    }

    async fn get_country_by_identifier(
        &self,
        identifier: &str,
    ) -> Result<Option<Country>, ServiceError> {
        let country = self
            .country_repository
            .fetch_country_by_identifier(identifier)
            .await
            .map_err(ServiceError::from)?;

        Ok(country)
    }
}
