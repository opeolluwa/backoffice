use async_graphql::dynamic::*;
use sea_orm::DatabaseConnection;
use seaography::{Builder, BuilderContext, async_graphql, lazy_static::lazy_static};

use super::mutations;
use crate::{api::state::AppState, entities::*};

lazy_static! {
    static ref CONTEXT: BuilderContext = BuilderContext::default();
}

pub fn schema(
    database: DatabaseConnection,
    depth: Option<usize>,
    complexity: Option<usize>,
    app_state: AppState,
) -> Result<Schema, SchemaError> {
    schema_builder(&CONTEXT, database, depth, complexity, app_state).finish()
}

pub fn schema_builder(
    context: &'static BuilderContext,
    database: DatabaseConnection,
    depth: Option<usize>,
    complexity: Option<usize>,
    app_state: AppState,
) -> SchemaBuilder {
    let mut builder = Builder::new(context, database.clone());
    builder = register_entity_modules(builder);

    seaography::register_custom_inputs!(
        builder,
        [super::types::newsletter::SubscribeToNewsletterInput]
    );
    seaography::register_custom_mutations!(
        builder,
        [
            mutations::newsletter::SubscribeNewsletter,
            mutations::health_check::HealthCheck
        ]
    );

    builder
        .set_depth_limit(depth)
        .set_complexity_limit(complexity)
        .schema_builder()
        .data(database)
        .data(app_state)
}
