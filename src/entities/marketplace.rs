use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(sqlx::FromRow, Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export, export_to = "Marketplace.d.ts")]
pub struct MarketPlace {
    pub identifier: String,
    pub name: String,
    pub slug: String,
    pub description: String,
    pub user_identifier: String,
    #[ts(type = "string")]
    pub created_at: DateTime<Local>,
    #[ts(type = "string")]
    pub updated_at: Option<DateTime<Local>>,
}
