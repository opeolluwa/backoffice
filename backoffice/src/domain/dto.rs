use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserProfile {
    pub identifier: String,
    pub email: String,
    pub first_name: String,
    pub last_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateUserCommand {
    pub email: String,
    pub password: String,
    pub first_name: String,
    pub last_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoginCommand {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForgottenPasswordCommand {
    pub email: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetNewPasswordCommand {
    pub password: String,
    pub confirm_password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerifyAccountCommand {
    pub otp: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RefreshTokenCommand {
    pub email: String,
    pub identifier: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenClaims {
    pub email: String,
    pub identifier: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoginResult {
    pub token: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForgottenPasswordResult {
    pub token: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetNewPasswordResult {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerifyAccountResult {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RefreshTokenResult {
    pub token: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateMarketplaceCommand {
    pub name: String,
    pub description: String,
    pub slug: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateEmailCommand {
    pub subject: String,
    pub body: String,
    pub sender_email: String,
    pub recipient_email: String,
    pub tag: Option<String>,
    pub has_attachments: Option<bool>,
    pub data: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateEmailCommand {
    pub tag: Option<String>,
    pub is_read: Option<bool>,
    pub is_starred: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateTeamMemberCommand {
    pub name: String,
    pub email: String,
    pub phone: Option<String>,
    pub role: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateTeamMemberCommand {
    pub name: Option<String>,
    pub phone: Option<String>,
    pub role: Option<String>,
    pub blocked: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateUploadCommand {
    pub name: Option<String>,
    pub starred: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SaveProductCommand {
    pub picture: String,
    pub name: String,
    pub description: String,
    pub price: i64,
    pub currency_identifier: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmailMessage {
    pub from_address: String,
    pub from_name: String,
    pub to_address: String,
    pub to_name: String,
    pub subject: String,
    pub html_body: String,
}
