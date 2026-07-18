mod client;
mod error;
pub use client::*;
pub use error::*;

use std::path::Path;
use backoffice_domain::ports::image_uploader::{ImageUploader, UploadResult};
use backoffice_domain::errors::service_error::ServiceError;

impl ImageUploader for ImagekitClient {
    async fn upload_file(&self, path: &Path, file_name: &str) -> Result<UploadResult, ServiceError> {
        let response = self.upload_file(path, file_name).await
            .map_err(|e| ServiceError::OperationFailed(e.to_string()))?;

        Ok(UploadResult {
            url: response.url,
            size: response.size,
        })
    }
}
