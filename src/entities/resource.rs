use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Resource {
    pub identifier: String,
    pub schema: Value,
    pub icon: String,
    pub created_at: DateTime<Local>,
    pub updated_at: DateTime<Local>,
}
