use sqlx::{FromRow, types::time::OffsetDateTime};
use ts_rs::TS;

#[derive(Debug, FromRow, TS)]
#[ts(export)]

pub struct App {
    pub identifier: i16,
    pub app_name: Option<String>,
    pub maintenance_mode: bool,
    pub support_email: Option<String>,
    #[ts(type = "string")]
    pub created_at: OffsetDateTime,
    #[ts(type = "string")]
    pub last_updated: OffsetDateTime,
}
