use crate::{
    dto::UpdateUploadCommand,
    models::uploads,
    ports::{
        image_uploader::ImageUploader,
        upload_repository::UploadRepositoryExt,
    },
    errors::service_error::ServiceError,
};

pub struct UploadsService<R: UploadRepositoryExt, U: ImageUploader> {
    repo: R,
    uploader: U,
}

impl<R: UploadRepositoryExt, U: ImageUploader> UploadsService<R, U> {
    pub fn new(repo: R, uploader: U) -> Self {
        Self { repo, uploader }
    }
}

pub trait UploadsServiceExt {
    async fn create_upload(
        &self,
        file_path: std::path::PathBuf,
        file_name: &str,
        name: &str,
        starred: bool,
    ) -> Result<uploads::Model, ServiceError>;

    async fn find_upload_by_identifier(
        &self,
        identifier: &str,
    ) -> Result<uploads::Model, ServiceError>;

    async fn find_all_uploads(&self) -> Result<Vec<uploads::Model>, ServiceError>;

    async fn find_starred_uploads(&self) -> Result<Vec<uploads::Model>, ServiceError>;

    async fn update_upload(
        &self,
        identifier: &str,
        command: &UpdateUploadCommand,
    ) -> Result<uploads::Model, ServiceError>;

    async fn delete_upload(&self, identifier: &str) -> Result<(), ServiceError>;

    async fn count_uploads(&self) -> Result<i64, ServiceError>;
}

impl<R: UploadRepositoryExt + Send + Sync, U: ImageUploader + Send + Sync> UploadsServiceExt for UploadsService<R, U> {
    async fn create_upload(
        &self,
        file_path: std::path::PathBuf,
        file_name: &str,
        name: &str,
        starred: bool,
    ) -> Result<uploads::Model, ServiceError> {
        let upload_response = self.uploader
            .upload_file(&file_path, file_name)
            .await?;

        let file_size = upload_response
            .size
            .try_into()
            .ok();

        let model = self
            .repo
            .create_upload(
                name,
                &upload_response.url,
                None, //TODO; use mime type to map file type  
                file_size,
                starred,
            )
            .await?;

        let _ = std::fs::remove_file(&file_path);

        Ok(model)
    }

    async fn find_upload_by_identifier(
        &self,
        identifier: &str,
    ) -> Result<uploads::Model, ServiceError> {
        Ok(self.repo.find_upload_by_identifier(identifier).await?)
    }

    async fn find_all_uploads(&self) -> Result<Vec<uploads::Model>, ServiceError> {
        Ok(self.repo.find_all_uploads().await?)
    }

    async fn find_starred_uploads(&self) -> Result<Vec<uploads::Model>, ServiceError> {
        Ok(self.repo.find_starred_uploads().await?)
    }

    async fn update_upload(
        &self,
        identifier: &str,
        command: &UpdateUploadCommand,
    ) -> Result<uploads::Model, ServiceError> {
        Ok(self.repo.update_upload(identifier, command).await?)
    }

    async fn delete_upload(&self, identifier: &str) -> Result<(), ServiceError> {
        Ok(self.repo.delete_upload(identifier).await?)
    }

    async fn count_uploads(&self) -> Result<i64, ServiceError> {
        Ok(self.repo.count_uploads().await?)
    }
}
