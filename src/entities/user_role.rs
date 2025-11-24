use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use ts_rs::TS;

#[derive(Debug, Serialize, Deserialize, FromRow, TS)]
#[ts(export)]

pub struct UserRole {
    pub identifier: String,
    #[ts(type = "string")]
    pub created_at: DateTime<Local>,
    #[ts(type = "string")]
    pub updated_at: Option<DateTime<Local>>,
    pub name: String,
    pub description: String,
}
