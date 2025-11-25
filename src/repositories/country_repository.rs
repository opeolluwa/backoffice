use std::sync::Arc;

use sqlx::PgPool;

use crate::{
    entities::country::Country, errors::repository_error::RepositoryError,
    repositories::base::Repository,
};

#[derive(Clone)]
pub struct CountryRepository {
    pool: Arc<PgPool>,
}

pub(crate) trait CountryRepositoryExt {
    async fn fetch_all_countries(&self) -> Result<Vec<Country>, RepositoryError>;

    async fn fetch_country_by_identifier(
        &self,
        identifier: &str,
    ) -> Result<Option<Country>, RepositoryError>;
}

impl Repository for CountryRepository {
    fn init(pool: &PgPool) -> Self {
        Self {
            pool: Arc::new(pool.clone()),
        }
    }
}

impl CountryRepositoryExt for CountryRepository {
    async fn fetch_all_countries(&self) -> Result<Vec<Country>, RepositoryError> {
        let countries = sqlx::query_as::<_, Country>("SELECT * FROM countries")
            .fetch_all(self.pool.as_ref())
            .await
            .map_err(RepositoryError::from)?;

        Ok(countries)
    }

    async fn fetch_country_by_identifier(
        &self,
        identifier: &str,
    ) -> Result<Option<Country>, RepositoryError> {
        let country = sqlx::query_as::<_, Country>("SELECT * FROM countries WHERE identifier = $1")
            .bind(identifier)
            .fetch_optional(self.pool.as_ref())
            .await
            .map_err(RepositoryError::from)?;

        Ok(country)
    }
}
