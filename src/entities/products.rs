use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use time::OffsetDateTime;
use ts_rs::TS;

#[derive(FromRow, Debug, Clone, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
pub struct Product {
    pub identifier: String,
    pub name: String,
    pub picture: Option<String>,
    #[ts(type = "number")]
    pub price: rust_decimal::Decimal,
    pub description: String,
    pub created_by_identifier: Option<String>,
    pub marketplace_identifier: Option<String>,
    #[ts(type = "string")]
    #[serde(with = "time::serde::iso8601")]
    pub created_at: OffsetDateTime,
    #[ts(type = "string")]
    #[serde(with = "time::serde::iso8601::option")]
    pub updated_at: Option<OffsetDateTime>,
}
