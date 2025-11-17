use std::path::Path;

use backoffice_utils::extract_env;
use sqlx::{PgPool, migrate::Migrator, postgres::PgPoolOptions};

use crate::errors::app_error::AppError;

pub struct AppDatabase {}

impl AppDatabase {
    pub async fn init_pool() -> Result<PgPool, AppError> {
        let database_url = extract_env::<String>("DATABASE_URL")?;

        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(&database_url)
            .await
            .map_err(|err| AppError::StartupError(err.to_string()))?;

        tracing::info!("Database pool initialized");

        let migrator = Migrator::new(Path::new("migrations"))
            .await
            .map_err(|err| AppError::StartupError(err.to_string()))?;
        migrator
            .run(&pool)
            .await
            .map_err(|err| AppError::StartupError(err.to_string()))?;

        Ok(pool)
    }
}
