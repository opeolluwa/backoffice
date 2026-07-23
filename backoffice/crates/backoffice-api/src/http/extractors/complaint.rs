use validator::Validate;

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone, Validate)]
#[serde(rename_all = "camelCase")]
pub struct CreateComplaintRequest {
    pub customer_identifier: String,
    pub order_identifier: Option<String>,
    pub subject: String,
    pub description: String,
}

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone, Validate)]
#[serde(rename_all = "camelCase")]
pub struct UpdateComplaintRequest {
    pub subject: Option<String>,
    pub description: Option<String>,
    pub status: Option<String>,
}
