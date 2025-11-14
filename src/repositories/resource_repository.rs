use crate::entities::resource::Resource;
use crate::errors::repository_error::RepositoryError;

pub(crate) trait ResourceExt {
    async fn create_resource(&self, resource: &Resource) -> Result<Resource, RepositoryError>;

    async fn find_resource_by_identifier(
        &self,
        identifier: &[u8],
    ) -> Result<Resource, RepositoryError>;

    async fn find_all_resources(&self) -> Result<Vec<Resource>, RepositoryError>;

    async fn update_resource_by_identifier(
        &self,
        identifier: &[u8],
        resource: &Resource,
    ) -> Result<Resource, RepositoryError>;

    async fn delete_resource_by_identifier(&self, identifier: &[u8])
    -> Result<(), RepositoryError>;

    async fn resource_exists(&self, identifier: &[u8]) -> Result<bool, RepositoryError>;

    async fn count_resources(&self) -> Result<i64, RepositoryError>;
    
    async fn find_resources_by_fields(
        &self,
        filters: &[(String, String)],
    ) -> Result<Vec<Resource>, RepositoryError>;

    async fn execute_raw_query(&self, query: &str) -> Result<(), RepositoryError>;
}
