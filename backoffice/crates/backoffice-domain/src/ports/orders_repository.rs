use crate::{
    dto::PlaceOrderCommand,
    errors::database_error::DatabaseError,
    models::{orders, products},
};

#[cfg_attr(test, mockall::automock)]
#[allow(async_fn_in_trait)]
pub trait OrdersRepositoryExt {
    async fn place_orders(
        &self,
        command: &PlaceOrderCommand,
    ) -> Result<Vec<(orders::Model, products::Model)>, DatabaseError>;

    async fn find_orders_by_identifier(
        &self,
        identifier: &str,
    ) -> Result<orders::Model, DatabaseError>;

    async fn find_all_orders(&self) -> Result<Vec<orders::Model>, DatabaseError>;

    async fn update_orders_by_identifier(
        &self,
        identifier: &str,
        command: &PlaceOrderCommand,
    ) -> Result<orders::Model, DatabaseError>;

    async fn delete_orders_by_identifier(&self, identifier: &str) -> Result<(), DatabaseError>;

    async fn count_orders(&self) -> Result<i64, DatabaseError>;
}
