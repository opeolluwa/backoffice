use crate::errors::service_error::ServiceError;
use crate::{
    dto::PlaceOrderCommand,
    models::{orders, products},
    ports::orders_repository::OrdersRepositoryExt,
};

#[derive(Clone)]
pub struct OrderService<R: OrdersRepositoryExt> {
    repo: R,
}

impl<R: OrdersRepositoryExt> OrderService<R> {
    pub fn new(repo: R) -> Self {
        Self { repo }
    }
}

pub trait OrderServiceExt {
    async fn place_orders(
        &self,
        command: &PlaceOrderCommand,
    ) -> Result<Vec<(orders::Model, products::Model)>, ServiceError>;

    async fn find_orders_by_identifier(
        &self,
        identifier: &str,
    ) -> Result<orders::Model, ServiceError>;

    async fn find_all_orders(&self) -> Result<Vec<orders::Model>, ServiceError>;

    async fn update_orders_by_identifier(
        &self,
        identifier: &str,
        command: &PlaceOrderCommand,
    ) -> Result<orders::Model, ServiceError>;

    async fn delete_orders_by_identifier(&self, identifier: &str) -> Result<(), ServiceError>;

    async fn count_orders(&self) -> Result<i64, ServiceError>;
}

impl<R: OrdersRepositoryExt + Send + Sync> OrderServiceExt for OrderService<R> {
    async fn place_orders(
        &self,
        command: &PlaceOrderCommand,
    ) -> Result<Vec<(orders::Model, products::Model)>, ServiceError> {
        Ok(self.repo.place_orders(command).await?)
    }

    async fn find_orders_by_identifier(
        &self,
        identifier: &str,
    ) -> Result<orders::Model, ServiceError> {
        Ok(self.repo.find_orders_by_identifier(identifier).await?)
    }

    async fn find_all_orders(&self) -> Result<Vec<orders::Model>, ServiceError> {
        Ok(self.repo.find_all_orders().await?)
    }

    async fn update_orders_by_identifier(
        &self,
        identifier: &str,
        command: &PlaceOrderCommand,
    ) -> Result<orders::Model, ServiceError> {
        Ok(self
            .repo
            .update_orders_by_identifier(identifier, command)
            .await?)
    }

    async fn delete_orders_by_identifier(&self, identifier: &str) -> Result<(), ServiceError> {
        Ok(self.repo.delete_orders_by_identifier(identifier).await?)
    }

    async fn count_orders(&self) -> Result<i64, ServiceError> {
        Ok(self.repo.count_orders().await?)
    }
}
