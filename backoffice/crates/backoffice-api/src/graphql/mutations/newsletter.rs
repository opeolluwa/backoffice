use seaography::itertools::Itertools;
use seaography::{
    CustomFields,
    async_graphql::{self, Context},
};
use validator::Validate;

use backoffice_domain::models::newsletter;
use backoffice_domain::services::newsletter::NewsletterServiceExt;
use backoffice_domain::{errors::app_error::AppError, utils::error::format_validation_errors};

use crate::graphql::types::newsletter::SubscribeToNewsletterInput;
use crate::state::AppState;

pub struct SubscribeNewsletter;

#[CustomFields]
impl SubscribeNewsletter {
    async fn subscribe_to_newsletter(
        ctx: &Context<'_>,
        input: SubscribeToNewsletterInput,
    ) -> async_graphql::Result<newsletter::Model> {
        if let Err(err) = input.validate() {
            let better_error_message = format_validation_errors(err);
            return Err(AppError::GraphQLError(better_error_message.into_iter().join(",")).into());
        }

        let state = ctx
            .data::<AppState>()
            .map_err(|err| AppError::GraphQLError(err.message))?;

        let model = state
            .services
            .newsletter_service
            .subscribe(&input.email)
            .await?;

        Ok(model)
    }
}
