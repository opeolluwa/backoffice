use super::products::Product;
use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use sqlx::types::Json;
use time::OffsetDateTime;
use ts_rs::TS;
#[derive(sqlx::FromRow, Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export, export_to = "Marketplace.d.ts")]
#[serde(rename_all = "camelCase")]
pub struct MarketPlace {
    pub identifier: String,
    pub name: String,
    pub slug: String,
    pub description: String,
    pub user_identifier: String,
    #[ts(type = "string")]
    pub created_at: OffsetDateTime,
    #[ts(type = "string")]
    pub updated_at: Option<OffsetDateTime>,
}

#[derive(Debug, serde::Deserialize, Serialize,  FromRow)]
pub struct MarketplaceWithProducts {
    pub identifier: String,
    pub name: String,
    pub description: String,
    pub products: Json<Vec<Product>>,
    // #[ts(type = "string")]
    pub created_at: OffsetDateTime,
    // #[ts(type = "string")]
    pub updated_at: Option<OffsetDateTime>,
    pub user_identifier: Option<String>,
}
