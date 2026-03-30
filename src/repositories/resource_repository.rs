use crate::errors::database_error::DatabaseError;

#[allow(dead_code)]
pub(crate) trait ResourceExt<T> {
    async fn create_resource(&self, resource: &T) -> Result<T, DatabaseError>;

    async fn find_resource_by_identifier(&self, identifier: &[u8]) -> Result<T, DatabaseError>;

    async fn find_all_resources(&self) -> Result<Vec<T>, DatabaseError>;

    async fn update_resource_by_identifier(
        &self,
        identifier: &[u8],
        resource: &T,
    ) -> Result<T, DatabaseError>;

    async fn delete_resource_by_identifier(&self, identifier: &[u8]) -> Result<(), DatabaseError>;

    async fn resource_exists(&self, identifier: &[u8]) -> Result<bool, DatabaseError>;

    async fn count_resources(&self) -> Result<i64, DatabaseError>;

    async fn find_resources_by_fields(
        &self,
        filters: &[(String, String)],
    ) -> Result<Vec<T>, DatabaseError>;

    async fn execute_raw_query(&self, query: &str) -> Result<(), DatabaseError>;
}
