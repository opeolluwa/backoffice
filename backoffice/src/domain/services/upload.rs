use axum_typed_multipart::TypedMultipart;

use crate::{
    api::http::extractors::upload::{CreateUploadRequest, UpdateUploadRequest},
    domain::models::{uploads, sea_orm_active_enums::FileType},
    domain::ports::upload_repository::UploadRepositoryExt,
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
        request: TypedMultipart<CreateUploadRequest>,
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
        request: &UpdateUploadRequest,
    ) -> Result<uploads::Model, ServiceError>;

    async fn delete_upload(&self, identifier: &str) -> Result<(), ServiceError>;

    async fn count_uploads(&self) -> Result<i64, ServiceError>;
}

impl<R: UploadRepositoryExt + Send + Sync> UploadsServiceExt for UploadsService<R> {
    async fn create_upload(
        &self,
        TypedMultipart(CreateUploadRequest { file, name, file_type, starred }): TypedMultipart<CreateUploadRequest>,
    ) -> Result<uploads::Model, ServiceError> {
        let file_name = file
            .metadata
            .file_name
            .clone()
            .unwrap_or_else(|| "upload".to_string());

        let file_path = file
            .contents
            .path()
            .to_path_buf();

        let private_key: String = extract_env("IMAGEKIT_PRIVATE_KEY")
            .map_err(|err| ServiceError::OperationFailed(err.to_string()))?;
        let public_key: String = extract_env("IMAGEKIT_PUBLIC_KEY")
            .map_err(|err| ServiceError::OperationFailed(err.to_string()))?;

        let imagekit_client =
            ImagekitClient::new(&public_key, &private_key).map_err(ServiceError::from)?;

        let upload_response = imagekit_client
            .upload_file(&file_path, &file_name)
            .await
            .map_err(ServiceError::from)?;

        let file_size = upload_response
            .size
            .try_into()
            .ok();

        let file_type = file_type.and_then(|ft| match ft.to_lowercase().as_str() {
            "image" => Some(FileType::Image),
            "video" => Some(FileType::Video),
            "audio" => Some(FileType::Audio),
            "document" => Some(FileType::Document),
            _ => Some(FileType::Others),
        });

        let model = self
            .repo
            .create_upload(
                &name,
                &upload_response.url,
                file_type,
                file_size,
                starred.unwrap_or(false),
            )
            .await?;

        let _ = std::fs::remove_file(&file_path);

        Ok(model)
    }

    async fn find_upload_by_identifier(
        &self,
        identifier: &str,
    ) -> Result<uploads::Model, ServiceError> {
        self.repo
            .find_upload_by_identifier(identifier)
            .await
            .map_err(|e| ServiceError::OperationFailed(e.to_string()))
    }

    async fn find_all_uploads(&self) -> Result<Vec<uploads::Model>, ServiceError> {
        self.repo
            .find_all_uploads()
            .await
            .map_err(|e| ServiceError::OperationFailed(e.to_string()))
    }

    async fn find_starred_uploads(&self) -> Result<Vec<uploads::Model>, ServiceError> {
        self.repo
            .find_starred_uploads()
            .await
            .map_err(|e| ServiceError::OperationFailed(e.to_string()))
    }

    async fn update_upload(
        &self,
        identifier: &str,
        request: &UpdateUploadRequest,
    ) -> Result<uploads::Model, ServiceError> {
        self.repo
            .update_upload(identifier, request)
            .await
            .map_err(|e| ServiceError::OperationFailed(e.to_string()))
    }

    async fn delete_upload(&self, identifier: &str) -> Result<(), ServiceError> {
        self.repo
            .delete_upload(identifier)
            .await
            .map_err(|e| ServiceError::OperationFailed(e.to_string()))
    }

    async fn count_uploads(&self) -> Result<i64, ServiceError> {
        self.repo
            .count_uploads()
            .await
            .map_err(|e| ServiceError::OperationFailed(e.to_string()))
    }
}
