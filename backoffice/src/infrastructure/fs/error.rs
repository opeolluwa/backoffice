#[derive(Debug, thiserror::Error)]
pub enum FileStorageError {
    #[error("failed to save file: {0}")]
    SaveFailed(String),
    #[error("failed to delete file: {0}")]
    DeleteFailed(String),
    #[error("file not found: {0}")]
    NotFound(String),
    #[error("upload failed: {0}")]
    UploadFailed(String),
    #[error("invalid file type: {0}")]
    InvalidFileType(String),
}
