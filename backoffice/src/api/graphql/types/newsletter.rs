use seaography::{CustomInputType, async_graphql};
use serde::{Deserialize, Serialize};
use ulid::Ulid;
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, Validate, CustomInputType)]
pub struct SubscribeToNewsletterInput {
    #[validate(email(message = "Invalid email format"))]
    pub email: String,
}

impl Into<crate::entities::newsletter::ActiveModel> for SubscribeToNewsletterInput {
    fn into(self) -> crate::entities::newsletter::ActiveModel {
        crate::entities::newsletter::ActiveModel {
            identifier: sea_orm::ActiveValue::Set(Ulid::new().to_string()),
            email: sea_orm::ActiveValue::Set(self.email),
            ..Default::default()
        }
    }
}
