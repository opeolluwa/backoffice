use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use time::OffsetDateTime;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Product {
    pub identifier: String,
    pub name: String,
    pub picture: Option<String>,
    pub price: rust_decimal::Decimal,
    pub description: String,
    pub created_by_identifier: Option<String>,
    pub marketplace_identifier: Option<String>,
    pub created_at: OffsetDateTime,
    pub updated_at: Option<OffsetDateTime>,
}
