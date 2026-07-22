use async_graphql::dynamic::*;
use seaography::{Builder, BuilderContext, async_graphql, lazy_static::lazy_static};

use backoffice_domain::models::*;

use super::mutations;
use super::types as GraphQLTypes;
use crate::state::AppState;

lazy_static! {
    static ref CONTEXT: BuilderContext = BuilderContext::default();
}

pub fn schema(
    depth: Option<usize>,
    complexity: Option<usize>,
    app_state: AppState,
) -> Result<Schema, SchemaError> {
    schema_builder(&CONTEXT, depth, complexity, app_state).finish()
}

pub fn schema_builder(
    context: &'static BuilderContext,
    depth: Option<usize>,
    complexity: Option<usize>,
    app_state: AppState,
) -> SchemaBuilder {
    let mut builder = Builder::new(context, app_state.database_connection.clone());
    builder = register_entity_modules(builder);
    builder = register_active_enums(builder);
    builder = register_active_enums(builder);

    seaography::register_custom_inputs!(
        builder,
        [
            GraphQLTypes::newsletter::SubscribeToNewsletterInput,
            GraphQLTypes::emails::SendEmailInput,
            GraphQLTypes::orders::PlaceOrderItemInput
        ]
    );

    seaography::register_custom_outputs!(builder, [GraphQLTypes::orders::PlaceOrderOrderResponse]);

    seaography::register_custom_mutations!(
        builder,
        [
            mutations::newsletter::SubscribeNewsletter,
            mutations::health_check::HealthCheck,
            mutations::emails::SendEmail,
            mutations::orders::PlaceOrders
        ]
    );

    builder
        .set_depth_limit(depth)
        .set_complexity_limit(complexity)
        .schema_builder()
        .data(app_state.clone().database_connection)
        .data(app_state)
}
