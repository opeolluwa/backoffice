use crate::{
    dto::CreateMarketplaceCommand,
    models::marketplaces,
    ports::marketplace_repository::MarketplaceRepositoryExt,
};
use crate::errors::service_error::ServiceError;

pub struct MarketplaceService<R: MarketplaceRepositoryExt> {
    repo: R,
}

impl<R: MarketplaceRepositoryExt> MarketplaceService<R> {
    pub fn new(repo: R) -> Self {
        Self { repo }
    }
}

pub trait MarketplaceServiceExt {
    async fn create_marketplace(
        &self,
        command: &CreateMarketplaceCommand,
        user_identifier: &str,
    ) -> Result<marketplaces::Model, ServiceError>;

    async fn find_marketplace_by_identifier(
        &self,
        identifier: &str,
        user_identifier: &str,
    ) -> Result<marketplaces::Model, ServiceError>;

    async fn find_all_marketplaces(
        &self,
        user_identifier: &str,
    ) -> Result<Vec<marketplaces::Model>, ServiceError>;

    async fn update_marketplace_by_identifier(
        &self,
        identifier: &str,
        command: &CreateMarketplaceCommand,
        user_identifier: &str,
    ) -> Result<marketplaces::Model, ServiceError>;

    async fn delete_marketplace_by_identifier(
        &self,
        identifier: &str,
        user_identifier: &str,
    ) -> Result<(), ServiceError>;

    async fn count_marketplaces(&self, user_identifier: &str) -> Result<i64, ServiceError>;
}

impl<R: MarketplaceRepositoryExt + Send + Sync> MarketplaceServiceExt for MarketplaceService<R> {
    async fn create_marketplace(
        &self,
        command: &CreateMarketplaceCommand,
        user_identifier: &str,
    ) -> Result<marketplaces::Model, ServiceError> {
        Ok(self.repo.create_marketplace(command, user_identifier).await?)
    }

    async fn find_marketplace_by_identifier(
        &self,
        identifier: &str,
        user_identifier: &str,
    ) -> Result<marketplaces::Model, ServiceError> {
        Ok(self
            .repo
            .find_marketplace_by_identifier(identifier, user_identifier)
            .await?)
    }

    async fn find_all_marketplaces(
        &self,
        user_identifier: &str,
    ) -> Result<Vec<marketplaces::Model>, ServiceError> {
        Ok(self.repo.find_all_marketplaces(user_identifier).await?)
    }

    async fn update_marketplace_by_identifier(
        &self,
        identifier: &str,
        command: &CreateMarketplaceCommand,
        user_identifier: &str,
    ) -> Result<marketplaces::Model, ServiceError> {
        Ok(self
            .repo
            .update_marketplace_by_identifier(identifier, command, user_identifier)
            .await?)
    }

    async fn delete_marketplace_by_identifier(
        &self,
        identifier: &str,
        user_identifier: &str,
    ) -> Result<(), ServiceError> {
        Ok(self
            .repo
            .delete_marketplace_by_identifier(identifier, user_identifier)
            .await?)
    }

    async fn count_marketplaces(&self, user_identifier: &str) -> Result<i64, ServiceError> {
        Ok(self.repo.count_marketplaces(user_identifier).await?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ports::marketplace_repository::MockMarketplaceRepositoryExt;
    use sea_orm::sqlx::types::chrono::Utc;

    fn test_marketplace() -> marketplaces::Model {
        marketplaces::Model {
            identifier: "mp-001".to_string(),
            name: "My Marketplace".to_string(),
            slug: "my-marketplace".to_string(),
            description: "A test marketplace".to_string(),
            user_identifier: Some("user-001".to_string()),
            created_at: Utc::now().naive_utc().and_utc().into(),
            updated_at: None,
        }
    }

    #[tokio::test]
    async fn create_marketplace_returns_model() {
        let mut repo = MockMarketplaceRepositoryExt::new();
        let mp = test_marketplace();
        repo.expect_create_marketplace()
            .returning(move |_, _| Ok(mp.clone()));
        let service = MarketplaceService::new(repo);

        let cmd = crate::dto::CreateMarketplaceCommand {
            name: "My Marketplace".to_string(),
            description: "A test marketplace".to_string(),
            slug: "my-marketplace".to_string(),
        };
        let result = service.create_marketplace(&cmd, "user-001").await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap().name, "My Marketplace");
    }

    #[tokio::test]
    async fn find_marketplace_by_identifier_found() {
        let mut repo = MockMarketplaceRepositoryExt::new();
        let mp = test_marketplace();
        repo.expect_find_marketplace_by_identifier()
            .returning(move |_, _| Ok(mp.clone()));
        let service = MarketplaceService::new(repo);

        let result = service.find_marketplace_by_identifier("mp-001", "user-001").await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn find_all_marketplaces() {
        let mut repo = MockMarketplaceRepositoryExt::new();
        repo.expect_find_all_marketplaces()
            .returning(|_| Ok(vec![test_marketplace(), test_marketplace()]));
        let service = MarketplaceService::new(repo);

        let result = service.find_all_marketplaces("user-001").await;
        assert_eq!(result.unwrap().len(), 2);
    }

    #[tokio::test]
    async fn count_marketplaces() {
        let mut repo = MockMarketplaceRepositoryExt::new();
        repo.expect_count_marketplaces().returning(|_| Ok(5));
        let service = MarketplaceService::new(repo);

        assert_eq!(service.count_marketplaces("user-001").await.unwrap(), 5);
    }

    #[tokio::test]
    async fn delete_marketplace_succeeds() {
        let mut repo = MockMarketplaceRepositoryExt::new();
        repo.expect_delete_marketplace_by_identifier()
            .returning(|_, _| Ok(()));
        let service = MarketplaceService::new(repo);

        assert!(service.delete_marketplace_by_identifier("mp-001", "user-001").await.is_ok());
    }
}
