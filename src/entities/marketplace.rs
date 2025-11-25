use super::products::Product;
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use sqlx::types::Json;
use time::OffsetDateTime;
use ts_rs::TS;
#[derive(sqlx::FromRow, Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export)]
#[serde(rename_all = "camelCase")]
pub struct MarketPlace {
    pub identifier: String,
    pub name: String,
    pub slug: String,
    pub description: String,
    pub user_identifier: String,
    #[ts(type = "string")]
    #[serde(with = "time::serde::rfc3339")]
    pub created_at: OffsetDateTime,
    #[ts(type = "string")]
    #[serde(with = "time::serde::rfc3339::option")]
    pub updated_at: Option<OffsetDateTime>,
}

#[derive(Debug, serde::Deserialize, Serialize, FromRow, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]

pub struct MarketplaceWithProducts {
    pub identifier: String,
    pub user_identifier: Option<String>,
    #[ts(type = "string")]
    #[serde(with = "time::serde::rfc3339")]
    pub created_at: OffsetDateTime,
    #[ts(type = "string")]
    #[serde(with = "time::serde::rfc3339::option")]
    pub updated_at: Option<OffsetDateTime>,
    pub name: String,
    pub description: String,
    #[ts(type = "array")]
    pub products: Json<Vec<Product>>,
}
