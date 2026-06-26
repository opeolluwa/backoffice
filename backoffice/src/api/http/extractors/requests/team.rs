use validator::Validate;

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone, Validate)]
#[serde(rename_all = "camelCase")]
pub struct CreateTeamMemberRequest {
    #[validate(length(min = 1))]
    pub name: String,
    #[validate(email)]
    pub email: String,
    pub phone: Option<String>,
    pub role: Option<String>,
}

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone, Validate)]
#[serde(rename_all = "camelCase")]
pub struct UpdateTeamMemberRequest {
    pub name: Option<String>,
    pub phone: Option<String>,
    pub role: Option<String>,
    pub blocked: Option<bool>,
}
