use std::time::Duration;

use sea_orm::{ConnectOptions, DatabaseConnection};

use backoffice_utils::extract_env;
use migration::{Migrator, MigratorTrait};

use crate::errors::app_error::AppError;

pub struct AppDatabase {}

impl AppDatabase {
    pub async fn init_pool() -> Result<DatabaseConnection, AppError> {
        let database_url = extract_env::<String>("DATABASE_URL")?;

        let mut opt = ConnectOptions::new(database_url);
        opt.max_connections(100)
            .min_connections(5)
            .connect_timeout(Duration::from_secs(8))
            .acquire_timeout(Duration::from_secs(8))
            .idle_timeout(Duration::from_secs(8))
            .max_lifetime(Duration::from_secs(8))
            .sqlx_logging(false) // disable SQLx logging
            .sqlx_logging_level(log::LevelFilter::Info); // set default Postgres schema

        let db = sea_orm::Database::connect(opt).await.map_err(|err| {
            log::error!("Failed to connect to the database: {}", err);
            AppError::StartupError("Failed to connect to the database".to_string())
        })?;

        Migrator::up(&db, None).await.map_err(|err| {
            log::error!(
                "failed to run database migration due to {}",
                err.to_string()
            );
            AppError::StartupError(err.to_string())
        })?;

        tracing::info!("Database pool initialized");

        Ok(db)
    }
}
