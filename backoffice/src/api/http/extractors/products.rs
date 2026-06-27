use axum_typed_multipart::{FieldData, TryFromMultipart};
use serde::{Deserialize, Serialize};
use tempfile::NamedTempFile;
use validator::Validate;

#[derive(Debug, Validate, TryFromMultipart)]
#[try_from_multipart(rename_all = "camelCase")]
pub struct CreateProductRequest {
    #[form_data(limit = "25MiB")]
    pub picture: FieldData<NamedTempFile>,
    pub name: String,
    pub description: String,
    pub price: i64,
    pub currency_identifier: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SaveProductRequest {
    pub picture: String,
    pub name: String,
    pub description: String,
    pub price: i64,
    pub currency_identifier: String,
}
