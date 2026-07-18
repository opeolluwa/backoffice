use crate::{
    models::invitation,
    ports::invitation_repository::InvitationRepositoryExt,
    errors::service_error::ServiceError,
};

pub struct InvitationService<R: InvitationRepositoryExt> {
    repo: R,
}

impl<R: InvitationRepositoryExt> InvitationService<R> {
    pub fn new(repo: R) -> Self {
        Self { repo }
    }
}

pub trait InvitationServiceExt {
    async fn create_invitation(
        &self,
        email: &str,
    ) -> Result<invitation::Model, ServiceError>;

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

    async fn block_invitation(
        &self,
        identifier: &str,
    ) -> Result<invitation::Model, ServiceError>;

    async fn delete_invitation(&self, identifier: &str) -> Result<(), ServiceError>;

    async fn count_invitations(&self) -> Result<i64, ServiceError>;
}

impl<R: InvitationRepositoryExt + Send + Sync> InvitationServiceExt for InvitationService<R> {
    async fn create_invitation(
        &self,
        email: &str,
    ) -> Result<invitation::Model, ServiceError> {
        let token = generate_invitation_token();
        let invitation = self
            .repo
            .create_invitation(email, &token)
            .await?;

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
        let invitation = self
            .repo
            .find_invitation_by_identifier(identifier)
            .await?;

        if invitation.token != token {
            return Err(ServiceError::OperationFailed(
                "invalid invitation token".to_string(),
            ));
        }

        Ok(self.repo.accept_invitation(identifier).await?)
    }

    async fn block_invitation(
        &self,
        identifier: &str,
    ) -> Result<invitation::Model, ServiceError> {
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
