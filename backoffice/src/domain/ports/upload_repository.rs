use crate::{
    api::http::extractors::upload::{CreateUploadRequest, UpdateUploadRequest},
    entities::uploads,
    errors::database_error::DatabaseError,
};

pub(crate) trait UploadRepositoryExt {
    async fn create_upload(
        &self,
        request: &CreateUploadRequest,
        user_identifier: &str,
    ) -> Result<uploads::Model, DatabaseError>;

    async fn find_upload_by_identifier(
        &self,
        identifier: &str,
        user_identifier: &str,
    ) -> Result<uploads::Model, DatabaseError>;

    async fn find_all_uploads(
        &self,
        user_identifier: &str,
    ) -> Result<Vec<uploads::Model>, DatabaseError>;

    async fn find_starred_uploads(
        &self,
        user_identifier: &str,
    ) -> Result<Vec<uploads::Model>, DatabaseError>;

    async fn update_upload(
        &self,
        identifier: &str,
        request: &UpdateUploadRequest,
        user_identifier: &str,
    ) -> Result<uploads::Model, DatabaseError>;

    async fn delete_upload(
        &self,
        identifier: &str,
        user_identifier: &str,
    ) -> Result<(), DatabaseError>;

    async fn count_uploads(&self, user_identifier: &str) -> Result<i64, DatabaseError>;
}
