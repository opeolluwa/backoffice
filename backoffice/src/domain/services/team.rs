use crate::domain::{
    dto::{CreateTeamMemberCommand, UpdateTeamMemberCommand},
    models::teams,
    ports::team_repository::TeamRepositoryExt,
};
use crate::errors::service_error::ServiceError;

pub struct TeamService<R: TeamRepositoryExt> {
    repo: R,
}

impl<R: TeamRepositoryExt> TeamService<R> {
    pub fn new(repo: R) -> Self {
        Self { repo }
    }
}

pub(crate) trait TeamServiceExt {
    async fn create_team_member(
        &self,
        command: &CreateTeamMemberCommand,
    ) -> Result<teams::Model, ServiceError>;

    async fn find_team_member_by_identifier(
        &self,
        identifier: &str,
    ) -> Result<teams::Model, ServiceError>;

    async fn find_all_team_members(&self) -> Result<Vec<teams::Model>, ServiceError>;

    async fn update_team_member(
        &self,
        identifier: &str,
        command: &UpdateTeamMemberCommand,
    ) -> Result<teams::Model, ServiceError>;

    async fn delete_team_member(&self, identifier: &str) -> Result<(), ServiceError>;

    async fn block_team_member(
        &self,
        identifier: &str,
        blocked: bool,
    ) -> Result<teams::Model, ServiceError>;

    async fn count_team_members(&self) -> Result<i64, ServiceError>;
}

impl<R: TeamRepositoryExt + Send + Sync> TeamServiceExt for TeamService<R> {
    async fn create_team_member(
        &self,
        command: &CreateTeamMemberCommand,
    ) -> Result<teams::Model, ServiceError> {
        Ok(self.repo.create_team_member(command).await?)
    }

    async fn find_team_member_by_identifier(
        &self,
        identifier: &str,
    ) -> Result<teams::Model, ServiceError> {
        Ok(self.repo.find_team_member_by_identifier(identifier).await?)
    }

    async fn find_all_team_members(&self) -> Result<Vec<teams::Model>, ServiceError> {
        Ok(self.repo.find_all_team_members().await?)
    }

    async fn update_team_member(
        &self,
        identifier: &str,
        command: &UpdateTeamMemberCommand,
    ) -> Result<teams::Model, ServiceError> {
        Ok(self.repo.update_team_member(identifier, command).await?)
    }

    async fn delete_team_member(&self, identifier: &str) -> Result<(), ServiceError> {
        Ok(self.repo.delete_team_member(identifier).await?)
    }

    async fn block_team_member(
        &self,
        identifier: &str,
        blocked: bool,
    ) -> Result<teams::Model, ServiceError> {
        Ok(self.repo.block_team_member(identifier, blocked).await?)
    }

    async fn count_team_members(&self) -> Result<i64, ServiceError> {
        Ok(self.repo.count_team_members().await?)
    }
}
