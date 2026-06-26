// use redis;

#[derive(Debug, thiserror::Error)]
pub enum RedisError {
    #[error("redis connection failed: {0}")]
    ConnectionFailed(String),
    #[error("redis operation failed: {0}")]
    OperationFailed(String),
    #[error("key not found: {0}")]
    KeyNotFound(String),
    // #[error(transparent)]
    // ClientError(#[from] redis::RedisError),
}
