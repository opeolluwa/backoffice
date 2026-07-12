use validator::Validate;

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone, Validate)]
#[serde(rename_all = "camelCase")]
pub struct CreateInvitationRequest {
    #[validate(email)]
    pub email: String,
}

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone, Validate)]
#[serde(rename_all = "camelCase")]
pub struct AcceptInvitationRequest {
    #[validate(length(min = 1))]
    pub token: String,
}
