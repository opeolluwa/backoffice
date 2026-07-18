use validator::Validate;

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone, Validate)]
#[serde(rename_all = "camelCase")]
pub struct CreateMarketplaceRequest {
    pub name: String,
    pub description: String,
    pub slug: String,
}
