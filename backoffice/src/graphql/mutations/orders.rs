use seaography::itertools::Itertools;
use seaography::{
    CustomFields,
    async_graphql::{self, Context},
};
use validator::Validate;

use crate::dto::PlaceOrderCommand;
use crate::errors::app_error::AppError;
use crate::services::orders_services::OrderServiceExt;
use crate::utils::error::format_validation_errors;

use crate::graphql::types::orders::{PlaceOrderItemInput, PlaceOrderOrderResponse};
use crate::state::AppState;

pub struct PlaceOrders;

#[CustomFields]
impl PlaceOrders {
    async fn place_orders(
        ctx: &Context<'_>,
        input: Vec<PlaceOrderItemInput>,
    ) -> async_graphql::Result<Vec<PlaceOrderOrderResponse>> {
        for item in &input {
            if let Err(err) = item.validate() {
                let better_error_message = format_validation_errors(err);
                return Err(
                    AppError::GraphQLError(better_error_message.into_iter().join(",")).into(),
                );
            }
        }

        let state = ctx
            .data::<AppState>()
            .map_err(|err| AppError::GraphQLError(err.message))?;

        let command = PlaceOrderCommand {
            items: input.into_iter().map(Into::into).collect(),
        };

        let pairs = state.services.orders_service.place_orders(&command).await?;

        Ok(pairs.into_iter().map(Into::into).collect())
    }
}
