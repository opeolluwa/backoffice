use serde::Deserialize;
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct CreateProductRequest {
    pub picture: Option<String>,
    pub name: String,
    pub description: String,
    pub price: i64,
    pub currency_identifier: String,
}
