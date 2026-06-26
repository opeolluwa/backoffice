use std::path::PathBuf;

use crate::errors::filesystem_error::AppFileSystemError;

pub struct FileUpload {
    pub file_name: String,
    pub content_type: Option<String>,
    pub data: Vec<u8>,
}

pub struct StoredFile {
    pub file_name: String,
    pub file_path: PathBuf,
}

pub(crate) trait FileStoragePort {
    async fn store_file(&self, upload: FileUpload) -> Result<StoredFile, AppFileSystemError>;

    async fn delete_file(&self, path: &str) -> Result<(), AppFileSystemError>;
}
