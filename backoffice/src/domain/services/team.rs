use crate::{
    api::http::extractors::team::{CreateTeamMemberRequest, UpdateTeamMemberRequest},
    domain::ports::team_repository::TeamRepositoryExt,
    entities::teams,
    errors::service_error::ServiceError,
};

#[derive(Clone)]
pub struct TeamService<R: TeamRepositoryExt> {
    repo: R,
}

impl<R: TeamRepositoryExt + Clone> TeamService<R> {
    pub fn new(repo: R) -> Self {
        Self { repo }
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

impl<R: TeamRepositoryExt + Clone + Send + Sync> TeamServiceExt for TeamService<R> {
    async fn create_team_member(
        &self,
        request: &CreateTeamMemberRequest,
    ) -> Result<teams::Model, ServiceError> {
        self.repo
            .create_team_member(request)
            .await
            .map_err(|e| ServiceError::OperationFailed(e.to_string()))
    }

    async fn find_team_member_by_identifier(
        &self,
        identifier: &str,
    ) -> Result<teams::Model, ServiceError> {
        self.repo
            .find_team_member_by_identifier(identifier)
            .await
            .map_err(|e| ServiceError::OperationFailed(e.to_string()))
    }

    async fn find_all_team_members(&self) -> Result<Vec<teams::Model>, ServiceError> {
        self.repo
            .find_all_team_members()
            .await
            .map_err(|e| ServiceError::OperationFailed(e.to_string()))
    }

    async fn update_team_member(
        &self,
        identifier: &str,
        request: &UpdateTeamMemberRequest,
    ) -> Result<teams::Model, ServiceError> {
        self.repo
            .update_team_member(identifier, request)
            .await
            .map_err(|e| ServiceError::OperationFailed(e.to_string()))
    }

    async fn delete_team_member(&self, identifier: &str) -> Result<(), ServiceError> {
        self.repo
            .delete_team_member(identifier)
            .await
            .map_err(|e| ServiceError::OperationFailed(e.to_string()))
    }

    async fn block_team_member(
        &self,
        identifier: &str,
        blocked: bool,
    ) -> Result<teams::Model, ServiceError> {
        self.repo
            .block_team_member(identifier, blocked)
            .await
            .map_err(|e| ServiceError::OperationFailed(e.to_string()))
    }

    async fn count_team_members(&self) -> Result<i64, ServiceError> {
        self.repo
            .count_team_members()
            .await
            .map_err(|e| ServiceError::OperationFailed(e.to_string()))
    }
}
