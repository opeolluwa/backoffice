use crate::{
    domain::{
        dto::UpdateUploadCommand,
        models::uploads,
        ports::upload_repository::UploadRepositoryExt,
    },
    errors::service_error::ServiceError,
    infrastructure::imagekit::ImagekitClient,
    shared::extract_env::extract_env,
};

pub struct UploadsService<R: UploadRepositoryExt> {
    repo: R,
}

impl<R: UploadRepositoryExt> UploadsService<R> {
    pub fn new(repo: R) -> Self {
        Self { repo }
    }
}

pub(crate) trait UploadsServiceExt {
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

impl<R: UploadRepositoryExt + Send + Sync> UploadsServiceExt for UploadsService<R> {
    async fn create_upload(
        &self,
        file_path: std::path::PathBuf,
        file_name: &str,
        name: &str,
        starred: bool,
    ) -> Result<uploads::Model, ServiceError> {
        let private_key: String = extract_env("IMAGEKIT_PRIVATE_KEY")?;
        let public_key: String = extract_env("IMAGEKIT_PUBLIC_KEY")?;

        let imagekit_client =
            ImagekitClient::new(&public_key, &private_key).map_err(ServiceError::from)?;

        let upload_response = imagekit_client
            .upload_file(&file_path, file_name)
            .await
            .map_err(ServiceError::from)?;

        let file_size = upload_response
            .size
            .try_into()
            .ok();

        let model = self
            .repo
            .create_upload(
                name,
                &upload_response.url,
                None,
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
