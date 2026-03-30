use sea_orm::ActiveModelTrait;
use sea_orm::{DatabaseConnection, DbErr};
use seaography::itertools::Itertools;
use seaography::{
    CustomFields,
    async_graphql::{self, Context},
};
use validator::Validate;

use crate::{
    errors::app_error::AppError, types::newsletter::SubscribeToNewsletterInput,
    utils::error::format_validation_errors,
};

pub struct SubscribeNewsletter;

#[CustomFields]
impl SubscribeNewsletter {
    async fn subscribe_to_newsletter(
        ctx: &Context<'_>,
        input: SubscribeToNewsletterInput,
    ) -> async_graphql::Result<crate::entities::newsletter::Model> {
        if let Err(err) = input.validate() {
            let better_error_message = format_validation_errors(err);
            return Err(AppError::GraphQLError(better_error_message.into_iter().join(",")).into());
        }

        let db_conn = ctx
            .data::<DatabaseConnection>()
            .map_err(|err| AppError::GraphQLError(err.message))?;

        let active_model: crate::entities::newsletter::ActiveModel = input.into();

        let model: crate::entities::newsletter::Model =
            active_model.insert(db_conn).await.map_err(|err: DbErr| {
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
