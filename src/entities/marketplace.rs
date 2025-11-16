use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

#[derive(sqlx::FromRow, Debug, Clone, Serialize, Deserialize)]
pub struct MarketPlace {
    pub identifier: String,
    pub name: String,
    pub slug: String,
    pub description: String,
    pub user_identifier: String,
    pub created_at: DateTime<Local>,
    pub updated_at: Option<DateTime<Local>>,
}
