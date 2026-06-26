#[derive(Debug, thiserror::Error)]
pub enum DatabaseError {
    #[error("record not found: {0}")]
    NotFound(String),
    #[error("duplicate record: {0}")]
    DuplicateEntry(String),
    #[error("invalid data: {0}")]
    InvalidData(String),
    #[error("database operation failed: {0}")]
    OperationFailed(String),
    #[error(transparent)]
    SeaOrmError(#[from] sea_orm::DbErr),
    #[error("a user with the same email already exists")]
    DuplicateEmailForUser,
}
