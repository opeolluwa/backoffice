use backoffice_domain::models::sea_orm_active_enums::OrderStatus;
use rust_decimal::Decimal;
use seaography::{CustomInputType, CustomOutputType};
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Clone, Serialize, Deserialize, CustomInputType, Validate)]
#[serde(rename_all = "camelCase")]
pub struct PlaceOrderItemInput {
    pub product_identifier: String,
    #[validate(range(min = 1))]
    pub quantity: i32,
}

#[derive(Debug, Clone, CustomOutputType)]
pub struct PlaceOrderOrderResponse {
    pub quantity: i32,
    pub status: Option<OrderStatus>,
    pub name: String,
    pub description: String,
    pub price: Decimal,
    pub currency_identifier: Option<String>,
    pub picture: Option<String>,
}

impl
    From<(
        backoffice_domain::models::orders::Model,
        backoffice_domain::models::products::Model,
    )> for PlaceOrderOrderResponse
{
    fn from(
        (order, product): (
            backoffice_domain::models::orders::Model,
            backoffice_domain::models::products::Model,
        ),
    ) -> Self {
        Self {
            quantity: order.quantity,
            status: order.status,
            name: product.name,
            description: product.description,
            price: product.price,
            currency_identifier: product.currency_identifier,
            picture: product.picture,
        }
    }
}

impl From<PlaceOrderItemInput> for backoffice_domain::dto::PlaceOrderItem {
    fn from(val: PlaceOrderItemInput) -> Self {
        Self {
            product_identifier: val.product_identifier,
            quantity: val.quantity,
        }
    }
}
