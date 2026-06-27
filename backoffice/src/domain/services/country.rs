use crate::{
    domain::ports::country_repository::CountryRepositoryExt, entities::countries,
    errors::service_error::ServiceError,
};

pub struct CountryService<R: CountryRepositoryExt> {
    repo: R,
}

impl<R: CountryRepositoryExt> CountryService<R> {
    pub fn new(repo: R) -> Self {
        Self { repo }
    }
}

pub(crate) trait CountryServiceExt {
    async fn get_all_countries(&self) -> Result<Vec<countries::Model>, ServiceError>;

    async fn get_country_by_identifier(
        &self,
        identifier: &str,
    ) -> Result<Option<countries::Model>, ServiceError>;
}

impl<R: CountryRepositoryExt + Send + Sync> CountryServiceExt for CountryService<R> {
    async fn get_all_countries(&self) -> Result<Vec<countries::Model>, ServiceError> {
        self.repo
            .fetch_all_countries()
            .await
            .map_err(ServiceError::from)
    }

    async fn get_country_by_identifier(
        &self,
        identifier: &str,
    ) -> Result<Option<countries::Model>, ServiceError> {
        self.repo
            .fetch_country_by_identifier(identifier)
            .await
            .map_err(ServiceError::from)
    }
}
