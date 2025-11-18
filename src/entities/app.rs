
use sqlx::{FromRow, types::time::OffsetDateTime};

#[derive(Debug, FromRow)]
pub struct App {
    pub identifier: i16,
    pub app_name: Option<String>,
    pub maintenance_mode: bool,
    pub support_email: Option<String>,
    pub created_at: OffsetDateTime,
    pub last_updated: OffsetDateTime,
}
