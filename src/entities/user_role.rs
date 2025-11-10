use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct UserRole {
    pub identifier: String,
    pub created_at: DateTime<Local>,
    pub updated_at: Option<DateTime<Local>>,
    pub name: String,
    pub description: String,
}
