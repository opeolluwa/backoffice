use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::FromRow;
use ulid::Ulid;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Collection {
    pub name: String,
    pub identifier: Ulid,
    pub created_at: DateTime<Local>,
    pub updated_at: DateTime<Local>,
    pub schema: Value,
}
