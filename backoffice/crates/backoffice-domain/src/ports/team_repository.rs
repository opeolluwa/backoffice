use crate::{
    dto::{CreateTeamMemberCommand, UpdateTeamMemberCommand},
    models::teams,
    errors::database_error::DatabaseError,
};

#[allow(async_fn_in_trait)]
pub trait TeamRepositoryExt {
    async fn create_team_member(
        &self,
        command: &CreateTeamMemberCommand,
    ) -> Result<teams::Model, DatabaseError>;

    async fn find_team_member_by_identifier(
        &self,
        identifier: &str,
    ) -> Result<teams::Model, DatabaseError>;

    async fn find_all_team_members(&self) -> Result<Vec<teams::Model>, DatabaseError>;

    async fn update_team_member(
        &self,
        identifier: &str,
        command: &UpdateTeamMemberCommand,
    ) -> Result<teams::Model, DatabaseError>;

    async fn delete_team_member(&self, identifier: &str) -> Result<(), DatabaseError>;

    async fn block_team_member(
        &self,
        identifier: &str,
        blocked: bool,
    ) -> Result<teams::Model, DatabaseError>;

    async fn count_team_members(&self) -> Result<i64, DatabaseError>;
}
