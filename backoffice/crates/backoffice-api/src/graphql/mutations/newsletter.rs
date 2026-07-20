use backoffice_domain::models::newsletter;
use sea_orm::{ActiveModelTrait, DbErr};
use seaography::itertools::Itertools;
use seaography::{
    CustomFields,
    async_graphql::{self, Context},
};
use validator::Validate;

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

        let active_model: backoffice_domain::models::newsletter::ActiveModel = input.into();

        let model: newsletter::Model = active_model
            .insert(&state.database_connection)
            .await
            .map_err(|err: DbErr| {
                let msg = err.to_string();
                if msg.contains("UNIQUE constraint failed") || msg.contains("duplicate key") {
                    AppError::GraphQLError("This email is already subscribed".to_string())
                } else {
                    AppError::GraphQLError(msg)
                }
            })?;

        Ok(model)
    }
}
