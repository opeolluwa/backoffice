use sea_orm::DatabaseConnection;

use crate::{
    adapters::requests::team::{CreateTeamMemberRequest, UpdateTeamMemberRequest},
    entities::teams,
    errors::service_error::ServiceError,
    repositories::{
        base::Repository,
        team_repository::{TeamRepository, TeamRepositoryExt},
    },
};

#[derive(Clone)]
pub struct TeamService {
    team_repository: TeamRepository,
}

impl TeamService {
    pub fn init(db: &DatabaseConnection) -> Self {
        Self {
            team_repository: TeamRepository::init(db),
        }
    }
}

pub(crate) trait TeamServiceExt {
    async fn create_team_member(
        &self,
        request: &CreateTeamMemberRequest,
    ) -> Result<teams::Model, ServiceError>;

    async fn find_team_member_by_identifier(
        &self,
        identifier: &str,
    ) -> Result<teams::Model, ServiceError>;

    async fn find_all_team_members(&self) -> Result<Vec<teams::Model>, ServiceError>;

    async fn update_team_member(
        &self,
        identifier: &str,
        request: &UpdateTeamMemberRequest,
    ) -> Result<teams::Model, ServiceError>;

    async fn delete_team_member(&self, identifier: &str) -> Result<(), ServiceError>;

    async fn block_team_member(
        &self,
        identifier: &str,
        blocked: bool,
    ) -> Result<teams::Model, ServiceError>;

    async fn count_team_members(&self) -> Result<i64, ServiceError>;
}

impl TeamServiceExt for TeamService {
    async fn create_team_member(
        &self,
        request: &CreateTeamMemberRequest,
    ) -> Result<teams::Model, ServiceError> {
        self.team_repository
            .create_team_member(request)
            .await
            .map_err(|e| ServiceError::OperationFailed(e.to_string()))
    }

    async fn find_team_member_by_identifier(
        &self,
        identifier: &str,
    ) -> Result<teams::Model, ServiceError> {
        self.team_repository
            .find_team_member_by_identifier(identifier)
            .await
            .map_err(|e| ServiceError::OperationFailed(e.to_string()))
    }

    async fn find_all_team_members(&self) -> Result<Vec<teams::Model>, ServiceError> {
        self.team_repository
            .find_all_team_members()
            .await
            .map_err(|e| ServiceError::OperationFailed(e.to_string()))
    }

    async fn update_team_member(
        &self,
        identifier: &str,
        request: &UpdateTeamMemberRequest,
    ) -> Result<teams::Model, ServiceError> {
        self.team_repository
            .update_team_member(identifier, request)
            .await
            .map_err(|e| ServiceError::OperationFailed(e.to_string()))
    }

    async fn delete_team_member(&self, identifier: &str) -> Result<(), ServiceError> {
        self.team_repository
            .delete_team_member(identifier)
            .await
            .map_err(|e| ServiceError::OperationFailed(e.to_string()))
    }

    async fn block_team_member(
        &self,
        identifier: &str,
        blocked: bool,
    ) -> Result<teams::Model, ServiceError> {
        self.team_repository
            .block_team_member(identifier, blocked)
            .await
            .map_err(|e| ServiceError::OperationFailed(e.to_string()))
    }

    async fn count_team_members(&self) -> Result<i64, ServiceError> {
        self.team_repository
            .count_team_members()
            .await
            .map_err(|e| ServiceError::OperationFailed(e.to_string()))
    }
}
