use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};

use crate::errors::database_error::DatabaseError;
use crate::models::countries::{self, Entity as CountryEntity};

use crate::repositories::base::Repository;

#[cfg_attr(test, mockall::automock)]
#[allow(async_fn_in_trait)]
pub trait CountryRepositoryExt {
    async fn fetch_all_countries(&self) -> Result<Vec<countries::Model>, DatabaseError>;

    async fn fetch_country_by_identifier(
        &self,
        identifier: &str,
    ) -> Result<Option<countries::Model>, DatabaseError>;
}

#[derive(Clone)]
pub struct CountryRepository {
    db: DatabaseConnection,
}

impl Repository for CountryRepository {
    fn init(db: &DatabaseConnection) -> Self {
        Self { db: db.clone() }
    }
}

impl CountryRepositoryExt for CountryRepository {
    async fn fetch_all_countries(&self) -> Result<Vec<countries::Model>, DatabaseError> {
        let countries = CountryEntity::find()
            .all(&self.db)
            .await
            .map_err(DatabaseError::from)?;
        Ok(countries)
    }

    async fn fetch_country_by_identifier(
        &self,
        identifier: &str,
    ) -> Result<Option<countries::Model>, DatabaseError> {
        let country = CountryEntity::find()
            .filter(countries::Column::Identifier.eq(identifier))
            .one(&self.db)
            .await
            .map_err(DatabaseError::from)?;
        Ok(country)
    }
}
