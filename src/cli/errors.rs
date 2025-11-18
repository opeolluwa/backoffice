
#[derive(thiserror::Error, Debug)]
pub enum CliError {
    #[error("Parse Error: {0}")]
    ParseError(String),
    #[error("Command Error: {0}")]
    CommandError(String),
    #[error("Config error: {0}")]
    ConfigError(String),
    #[error("Database error: {0}")]
    DatabaseError(String),
    #[error(transparent)]
    SqlxError(#[from] sqlx::Error),
        #[error("Operation failed: {0}")]
    OperationFailed(String),
}
