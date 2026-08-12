use validator::Validate;

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone, Validate)]
#[serde(rename_all = "camelCase")]
pub struct CreateOrdersRequest {
    pub product_identifier: String,
    pub quantity: i32,
}
