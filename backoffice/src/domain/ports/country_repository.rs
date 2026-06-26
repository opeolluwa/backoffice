pub trait CountryRepositoryExt {
    async fn fetch_all_countries(&self) -> Result<Vec<CountryModel>, DatabaseError>;

    async fn fetch_country_by_identifier(
        &self,
        identifier: &str,
    ) -> Result<CountryModel, DatabaseError>;
}
