use serde_json::Value;
use validator::Validate;

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone, Validate)]
#[serde(rename_all = "camelCase")]
pub struct CreateEmailRequest {
    #[validate(length(min = 1))]
    pub subject: String,
    #[validate(length(min = 1))]
    pub body: String,
    #[validate(email)]
    pub sender_email: String,
    #[validate(email)]
    pub recipient_email: String,
    pub tag: Option<String>,
    pub has_attachments: Option<bool>,
    pub data: Option<Value>,
}

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone, Validate)]
#[serde(rename_all = "camelCase")]
pub struct UpdateEmailRequest {
    pub tag: Option<String>,
    pub is_read: Option<bool>,
    pub is_starred: Option<bool>,
}
