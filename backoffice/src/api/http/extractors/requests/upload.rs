use validator::Validate;

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone, Validate)]
#[serde(rename_all = "camelCase")]
pub struct CreateUploadRequest {
    #[validate(length(min = 1))]
    pub name: String,
    #[validate(length(min = 1))]
    pub src: String,
    pub file_type: Option<String>,
    pub size: Option<i64>,
    pub starred: Option<bool>,
}

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone, Validate)]
#[serde(rename_all = "camelCase")]
pub struct UpdateUploadRequest {
    pub name: Option<String>,
    pub starred: Option<bool>,
}
