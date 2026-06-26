use crate::{
    api::http::extractors::requests::team::{CreateTeamMemberRequest, UpdateTeamMemberRequest},
    entities::teams,
    errors::database_error::DatabaseError,
};

pub(crate) trait TeamRepositoryExt {
    async fn create_team_member(
        &self,
        request: &CreateTeamMemberRequest,
    ) -> Result<teams::Model, DatabaseError>;

    async fn find_team_member_by_identifier(
        &self,
        identifier: &str,
    ) -> Result<teams::Model, DatabaseError>;

    async fn find_all_team_members(&self) -> Result<Vec<teams::Model>, DatabaseError>;

    async fn update_team_member(
        &self,
        identifier: &str,
        request: &UpdateTeamMemberRequest,
    ) -> Result<teams::Model, DatabaseError>;

    async fn delete_team_member(&self, identifier: &str) -> Result<(), DatabaseError>;

    async fn block_team_member(
        &self,
        identifier: &str,
        blocked: bool,
    ) -> Result<teams::Model, DatabaseError>;

    async fn count_team_members(&self) -> Result<i64, DatabaseError>;
}
