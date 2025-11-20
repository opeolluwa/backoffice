use axum_typed_multipart::{FieldData, TryFromMultipart};
use tempfile::NamedTempFile;
use validator::Validate;

#[derive( Debug, Validate)]
#[derive(TryFromMultipart)]
#[try_from_multipart(rename_all = "camelCase")]
pub struct CreateProductRequest {
    #[form_data(limit = "25MiB")]
    pub document: FieldData<NamedTempFile>,
    pub picture: FieldData<NamedTempFile>,
    pub name: String,
    pub description: String,
    pub price: i64,
}
