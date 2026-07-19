use crate::{
    dto::UpdateUploadCommand,
    errors::database_error::DatabaseError,
    models::{sea_orm_active_enums::FileType, uploads},
};

#[cfg_attr(test, mockall::automock)]
#[allow(async_fn_in_trait)]
pub trait UploadRepositoryExt {
    async fn create_upload(
        &self,
        name: &str,
        url: &str,
        file_type: Option<FileType>,
        file_size: Option<i64>,
        starred: bool,
        file_path: &str,
        thumbnail_url: &str,
    ) -> Result<uploads::Model, DatabaseError>;

    async fn find_upload_by_identifier(
        &self,
        identifier: &str,
    ) -> Result<uploads::Model, DatabaseError>;

    async fn find_all_uploads(&self) -> Result<Vec<uploads::Model>, DatabaseError>;

    async fn find_starred_uploads(&self) -> Result<Vec<uploads::Model>, DatabaseError>;

    async fn update_upload(
        &self,
        identifier: &str,
        command: &UpdateUploadCommand,
    ) -> Result<uploads::Model, DatabaseError>;

    async fn delete_upload(&self, identifier: &str) -> Result<(), DatabaseError>;

    async fn count_uploads(&self) -> Result<i64, DatabaseError>;
}
