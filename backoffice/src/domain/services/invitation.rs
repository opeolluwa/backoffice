use crate::{
    domain::models::invitation,
    domain::ports::invitation_repository::InvitationRepositoryExt,
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

pub(crate) trait InvitationServiceExt {
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
            .await
            .map_err(|e| ServiceError::OperationFailed(e.to_string()))?;

        mock_send_invitation_email(email, &token).await;

        Ok(invitation)
    }

    async fn find_invitation_by_identifier(
        &self,
        identifier: &str,
    ) -> Result<invitation::Model, ServiceError> {
        self.repo
            .find_invitation_by_identifier(identifier)
            .await
            .map_err(|e| ServiceError::OperationFailed(e.to_string()))
    }

    async fn find_all_invitations(&self) -> Result<Vec<invitation::Model>, ServiceError> {
        self.repo
            .find_all_invitations()
            .await
            .map_err(|e| ServiceError::OperationFailed(e.to_string()))
    }

    async fn accept_invitation(
        &self,
        identifier: &str,
        token: &str,
    ) -> Result<invitation::Model, ServiceError> {
        let invitation = self
            .repo
            .find_invitation_by_identifier(identifier)
            .await
            .map_err(|e| ServiceError::OperationFailed(e.to_string()))?;

        if invitation.token != token {
            return Err(ServiceError::OperationFailed(
                "invalid invitation token".to_string(),
            ));
        }

        self.repo
            .accept_invitation(identifier)
            .await
            .map_err(|e| ServiceError::OperationFailed(e.to_string()))
    }

    async fn block_invitation(
        &self,
        identifier: &str,
    ) -> Result<invitation::Model, ServiceError> {
        self.repo
            .block_invitation(identifier)
            .await
            .map_err(|e| ServiceError::OperationFailed(e.to_string()))
    }

    async fn delete_invitation(&self, identifier: &str) -> Result<(), ServiceError> {
        self.repo
            .delete_invitation(identifier)
            .await
            .map_err(|e| ServiceError::OperationFailed(e.to_string()))
    }

    async fn count_invitations(&self) -> Result<i64, ServiceError> {
        self.repo
            .count_invitations()
            .await
            .map_err(|e| ServiceError::OperationFailed(e.to_string()))
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
