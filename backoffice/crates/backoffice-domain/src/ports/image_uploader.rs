use std::path::Path;

use crate::errors::service_error::ServiceError;

pub struct UploadResult {
    pub url: String,
    pub size: u64,
}

#[allow(async_fn_in_trait)]
pub trait ImageUploader: Send + Sync {
    async fn upload_file(
        &self,
        path: &Path,
        file_name: &str,
    ) -> Result<UploadResult, ServiceError>;
}
