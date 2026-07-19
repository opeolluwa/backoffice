use crate::{
    errors::service_error::ServiceError, models::invitation,
    ports::invitation_repository::InvitationRepositoryExt,
};

#[derive(Clone)]
pub struct InvitationService<R: InvitationRepositoryExt> {
    repo: R,
}

impl<R: InvitationRepositoryExt> InvitationService<R> {
    pub fn new(repo: R) -> Self {
        Self { repo }
    }
}

pub trait InvitationServiceExt {
    async fn create_invitation(&self, email: &str) -> Result<invitation::Model, ServiceError>;

    async fn find_invitation_by_identifier(
        &self,
        identifier: &str,
    ) -> Result<invitation::Model, ServiceError>;

    async fn find_all_invitations(&self) -> Result<Vec<invitation::Model>, ServiceError>;

    async fn accept_invitation(
        &self,
        identifier: &str,
        token: &str,
    ) -> Result<invitation::Model, ServiceError>;

    async fn block_invitation(&self, identifier: &str) -> Result<invitation::Model, ServiceError>;

    async fn delete_invitation(&self, identifier: &str) -> Result<(), ServiceError>;

    async fn count_invitations(&self) -> Result<i64, ServiceError>;
}

impl<R: InvitationRepositoryExt + Send + Sync> InvitationServiceExt for InvitationService<R> {
    async fn create_invitation(&self, email: &str) -> Result<invitation::Model, ServiceError> {
        let token = generate_invitation_token();
        let invitation = self.repo.create_invitation(email, &token).await?;

        mock_send_invitation_email(email, &token).await;

        Ok(invitation)
    }

    async fn find_invitation_by_identifier(
        &self,
        identifier: &str,
    ) -> Result<invitation::Model, ServiceError> {
        Ok(self.repo.find_invitation_by_identifier(identifier).await?)
    }

    async fn find_all_invitations(&self) -> Result<Vec<invitation::Model>, ServiceError> {
        Ok(self.repo.find_all_invitations().await?)
    }

    async fn accept_invitation(
        &self,
        identifier: &str,
        token: &str,
    ) -> Result<invitation::Model, ServiceError> {
        let invitation = self.repo.find_invitation_by_identifier(identifier).await?;

        if invitation.token != token {
            return Err(ServiceError::OperationFailed(
                "invalid invitation token".to_string(),
            ));
        }

        Ok(self.repo.accept_invitation(identifier).await?)
    }

    async fn block_invitation(&self, identifier: &str) -> Result<invitation::Model, ServiceError> {
        Ok(self.repo.block_invitation(identifier).await?)
    }

    async fn delete_invitation(&self, identifier: &str) -> Result<(), ServiceError> {
        Ok(self.repo.delete_invitation(identifier).await?)
    }

    async fn count_invitations(&self) -> Result<i64, ServiceError> {
        Ok(self.repo.count_invitations().await?)
    }
}

fn generate_invitation_token() -> String {
    use ulid::Ulid;
    Ulid::new().to_string()
}

async fn mock_send_invitation_email(email: &str, token: &str) {
    tracing::info!(
        "[MOCK EMAIL] Sending invitation to '{}' with token '{}'",
        email,
        token
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ports::invitation_repository::MockInvitationRepositoryExt;
    use sea_orm::sqlx::types::chrono::Utc;

    fn test_invitation(token: &str) -> crate::models::invitation::Model {
        crate::models::invitation::Model {
            identifier: uuid::Uuid::new_v4().to_string(),
            email: "invite@example.com".to_string(),
            status: Some(crate::models::sea_orm_active_enums::InvitationStatus::Pending),
            token: token.to_string(),
            created_at: Utc::now().naive_utc(),
            updated_at: None,
        }
    }

    #[tokio::test]
    async fn create_invitation_generates_token_and_creates() {
        let mut repo = MockInvitationRepositoryExt::new();
        repo.expect_create_invitation()
            .returning(|_email, token| Ok(test_invitation(token)));
        let service = InvitationService::new(repo);

        let result = service.create_invitation("invite@example.com").await;
        assert!(result.is_ok());
        let invitation = result.unwrap();
        assert!(!invitation.token.is_empty());
    }

    #[tokio::test]
    async fn accept_invitation_valid_token() {
        let mut repo = MockInvitationRepositoryExt::new();
        let token = "valid-token-123";
        repo.expect_find_invitation_by_identifier()
            .returning(move |_| Ok(test_invitation(token)));
        repo.expect_accept_invitation()
            .returning(move |_| Ok(test_invitation(token)));
        let service = InvitationService::new(repo);

        let result = service
            .accept_invitation("inv-001", "valid-token-123")
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn accept_invitation_invalid_token() {
        let mut repo = MockInvitationRepositoryExt::new();
        repo.expect_find_invitation_by_identifier()
            .returning(|_| Ok(test_invitation("correct-token")));
        let service = InvitationService::new(repo);

        let result = service.accept_invitation("inv-001", "wrong-token").await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn count_invitations() {
        let mut repo = MockInvitationRepositoryExt::new();
        repo.expect_count_invitations().returning(|| Ok(15));
        let service = InvitationService::new(repo);

        assert_eq!(service.count_invitations().await.unwrap(), 15);
    }

    #[tokio::test]
    async fn delete_invitation() {
        let mut repo = MockInvitationRepositoryExt::new();
        repo.expect_delete_invitation().returning(|_| Ok(()));
        let service = InvitationService::new(repo);

        assert!(service.delete_invitation("inv-001").await.is_ok());
    }
}
