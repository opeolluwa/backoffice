use axum_typed_multipart::{FieldData, TryFromMultipart};
use tempfile::NamedTempFile;
use validator::Validate;

#[derive(Debug, Validate, TryFromMultipart)]
#[try_from_multipart(rename_all = "camelCase")]
pub struct CreateUploadRequest {
    #[form_data(limit = "25MiB")]
    pub file: FieldData<NamedTempFile>,
    #[validate(length(min = 1))]
    pub name: String,
    pub file_type: Option<String>,
    pub starred: Option<bool>,
}

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone, Validate)]
#[serde(rename_all = "camelCase")]
pub struct UpdateUploadRequest {
    pub name: Option<String>,
    pub starred: Option<bool>,
}
