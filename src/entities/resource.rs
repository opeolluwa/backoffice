use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::FromRow;
use ts_rs::TS;

#[derive(Debug, Serialize, Deserialize, FromRow, TS)]
#[ts(export)]

pub struct Resource {
    pub identifier: String,
    #[ts(type = "object")]
    pub schema: Value,
    pub icon: String,
    #[ts(type = "string")]
    pub created_at: DateTime<Local>,
    #[ts(type = "string")]
    pub updated_at: DateTime<Local>,
}
