use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(sqlx::FromRow, Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export)]
#[serde(rename_all = "camelCase")]
pub struct Country {
    pub identifier: String,
    pub currency_code: String,
    pub currency: String,
    pub country: String,
    pub flag: Option<String>,
}
