use seaography::{CustomInputType, async_graphql};
use serde::{Deserialize, Serialize};
use ulid::Ulid;
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, Validate, CustomInputType)]
pub struct SubscribeToNewsletterInput {
    #[validate(email(message = "Invalid email format"))]
    pub email: String,
}

impl From<SubscribeToNewsletterInput> for crate::domain::models::newsletter::ActiveModel {
    fn from(val: SubscribeToNewsletterInput) -> Self {
        crate::domain::models::newsletter::ActiveModel {
            identifier: sea_orm::ActiveValue::Set(Ulid::new().to_string()),
            email: sea_orm::ActiveValue::Set(val.email),
            ..Default::default()
        }
    }
}
