use crate::{
    api::http::extractors::upload::{CreateUploadRequest, UpdateUploadRequest},
    domain::models::uploads,
    domain::ports::upload_repository::UploadRepositoryExt,
    errors::service_error::ServiceError,
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
        request: &CreateUploadRequest,
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
        request: &CreateUploadRequest,
    ) -> Result<uploads::Model, ServiceError> {
        self.repo
            .create_upload(request, "")
            .await
            .map_err(|e| ServiceError::OperationFailed(e.to_string()))
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
