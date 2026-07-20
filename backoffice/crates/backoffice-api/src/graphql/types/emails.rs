use backoffice_domain::models;
use sanitizer::prelude::Sanitizer;
use sanitizer::prelude::*;
use sea_orm::ActiveValue::Set;
use seaography::{CustomInputType, async_graphql};
use serde::{Deserialize, Serialize};
use ulid::Ulid;
use validator::Validate;



#[derive(Debug, Clone, Serialize, Deserialize, CustomInputType, Validate, Sanitizer)]
#[serde(rename_all = "camelCase")]
pub struct SendEmailInput {
    #[sanitizer(trim)]
    #[validate(length(min = 1, max = 200))]
    pub subject: String,
    #[validate(length(min = 1, max = 1500))]
    pub body: String,
    #[sanitizer(trim, lower_case)]
    #[validate(email(message = "Invalid email format"))]
    pub sender_email: String,
    #[sanitizer(trim, lower_case)]
    #[validate(email(message = "Invalid email format"))]
    pub recipient_email: String,
    pub tag: Option<String>,
    pub has_attachments: bool,
  
}

impl From<SendEmailInput> for models::emails::ActiveModel {
    fn from(val: SendEmailInput) -> Self {
        models::emails::ActiveModel {
            identifier: sea_orm::ActiveValue::Set(Ulid::new().to_string()),
            subject: Set(val.subject),
            body: Set(val.body),
            sender_email: Set(val.sender_email),
            recipient_email: Set(val.recipient_email), //TODO:(opeolluwa): drop this

            tag: Set(val.tag),
            has_attachments: Set(val.has_attachments),
            ..Default::default()
        }
    }
}
