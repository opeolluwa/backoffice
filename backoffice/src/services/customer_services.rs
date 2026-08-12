use crate::{
    errors::service_error::ServiceError, models::customers,
    repositories::customer_repository::CustomerRepositoryExt,
};

#[derive(Clone)]
pub struct CustomerService<Repo: CustomerRepositoryExt> {
    repository: Repo,
}

impl<Repo: CustomerRepositoryExt> CustomerService<Repo> {
    pub fn new(repository: Repo) -> Self {
        Self { repository }
    }
}

pub trait CustomerServiceExt {
    async fn find_all_customers(&self) -> Result<Vec<customers::Model>, ServiceError>;

    async fn find_customer_by_identifier(
        &self,
        identifier: &str,
    ) -> Result<customers::Model, ServiceError>;

    async fn delete_customer_by_identifier(&self, identifier: &str) -> Result<(), ServiceError>;

    async fn count_customers(&self) -> Result<i64, ServiceError>;
}

impl<Repo: CustomerRepositoryExt + Sync + Send> CustomerServiceExt for CustomerService<Repo> {
    async fn find_all_customers(&self) -> Result<Vec<customers::Model>, ServiceError> {
        Ok(self.repository.find_all_customers().await?)
    }

    async fn find_customer_by_identifier(
        &self,
        identifier: &str,
    ) -> Result<customers::Model, ServiceError> {
        Ok(self
            .repository
            .find_customer_by_identifier(identifier)
            .await?)
    }

    async fn delete_customer_by_identifier(&self, identifier: &str) -> Result<(), ServiceError> {
        Ok(self
            .repository
            .delete_customer_by_identifier(identifier)
            .await?)
    }

    async fn count_customers(&self) -> Result<i64, ServiceError> {
        Ok(self.repository.count_customers().await?)
    }
}
