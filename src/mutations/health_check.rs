use seaography::{
    CustomFields,
    async_graphql::{self, Context},
};

pub struct HealthCheck;

#[CustomFields]
impl HealthCheck {
    async fn health_check(_ctx: &Context<'_>, name: String) -> async_graphql::Result<String> {
        Ok(format!("Hello, {}!", name))
    }
}
