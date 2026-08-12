use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserIdentifier {
    pub identifier: String,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct UpdateProfileRequest {
    #[validate(length(min = 1, message = "first name is required"))]
    pub first_name: String,
    #[validate(length(min = 1, message = "last name is required"))]
    pub last_name: String,
    pub username: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ChangePasswordRequest {
    #[validate(length(min = 1, message = "current password is required"))]
    pub current_password: String,
    #[validate(length(min = 8, message = "password must be at least 8 characters"))]
    pub new_password: String,
    #[validate(must_match(other = "new_password", message = "passwords do not match"))]
    pub confirm_password: String,
}
