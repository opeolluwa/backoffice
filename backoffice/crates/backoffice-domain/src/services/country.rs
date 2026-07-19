use crate::{
    models::countries, ports::country_repository::CountryRepositoryExt,
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

pub trait CountryServiceExt {
    async fn get_all_countries(&self) -> Result<Vec<countries::Model>, ServiceError>;

    async fn get_country_by_identifier(
        &self,
        identifier: &str,
    ) -> Result<Option<countries::Model>, ServiceError>;
}

impl<R: CountryRepositoryExt + Send + Sync> CountryServiceExt for CountryService<R> {
    async fn get_all_countries(&self) -> Result<Vec<countries::Model>, ServiceError> {
        Ok(self.repo.fetch_all_countries().await?)
    }

    async fn get_country_by_identifier(
        &self,
        identifier: &str,
    ) -> Result<Option<countries::Model>, ServiceError> {
        Ok(self.repo.fetch_country_by_identifier(identifier).await?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ports::country_repository::MockCountryRepositoryExt;

    fn test_country() -> countries::Model {
        countries::Model {
            identifier: "NG".to_string(),
            currency_code: "NGN".to_string(),
            currency: "Naira".to_string(),
            country: "Nigeria".to_string(),
            flag: Some("🇳🇬".to_string()),
        }
    }

    #[tokio::test]
    async fn get_all_countries_returns_list() {
        let mut repo = MockCountryRepositoryExt::new();
        repo.expect_fetch_all_countries()
            .returning(|| Ok(vec![test_country()]));
        let service = CountryService::new(repo);

        let result = service.get_all_countries().await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap().len(), 1);
    }

    #[tokio::test]
    async fn get_country_by_identifier_found() {
        let mut repo = MockCountryRepositoryExt::new();
        repo.expect_fetch_country_by_identifier()
            .returning(|_| Ok(Some(test_country())));
        let service = CountryService::new(repo);

        let result = service.get_country_by_identifier("NG").await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap().unwrap().country, "Nigeria");
    }

    #[tokio::test]
    async fn get_country_by_identifier_not_found() {
        let mut repo = MockCountryRepositoryExt::new();
        repo.expect_fetch_country_by_identifier()
            .returning(|_| Ok(None));
        let service = CountryService::new(repo);

        let result = service.get_country_by_identifier("XX").await;
        assert!(result.is_ok());
        assert!(result.unwrap().is_none());
    }
}
