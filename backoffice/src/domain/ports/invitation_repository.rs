use crate::{
    domain::models::invitation,
    errors::database_error::DatabaseError,
};

pub(crate) trait InvitationRepositoryExt {
    async fn create_invitation(
        &self,
        email: &str,
        token: &str,
    ) -> Result<invitation::Model, DatabaseError>;

    async fn find_invitation_by_identifier(
        &self,
        identifier: &str,
    ) -> Result<invitation::Model, DatabaseError>;

    async fn find_invitation_by_token(
        &self,
        token: &str,
    ) -> Result<invitation::Model, DatabaseError>;

    async fn find_all_invitations(&self) -> Result<Vec<invitation::Model>, DatabaseError>;

    async fn accept_invitation(
        &self,
        identifier: &str,
    ) -> Result<invitation::Model, DatabaseError>;

    async fn block_invitation(
        &self,
        identifier: &str,
    ) -> Result<invitation::Model, DatabaseError>;

    async fn delete_invitation(&self, identifier: &str) -> Result<(), DatabaseError>;

    async fn count_invitations(&self) -> Result<i64, DatabaseError>;
}
