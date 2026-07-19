use crate::{errors::database_error::DatabaseError, models::countries};

#[cfg_attr(test, mockall::automock)]
#[allow(async_fn_in_trait)]
pub trait CountryRepositoryExt {
    async fn fetch_all_countries(&self) -> Result<Vec<countries::Model>, DatabaseError>;

    async fn fetch_country_by_identifier(
        &self,
        identifier: &str,
    ) -> Result<Option<countries::Model>, DatabaseError>;
}
