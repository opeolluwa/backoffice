#[derive(thiserror::Error, Debug)]
pub enum CliError {
    #[error("Parse error: {0}")]
    ParseError(String),

    #[error("IO error: {0}")]
    IoError(String),

    #[error("Generator error: {0}")]
    GeneratorError(String),

    #[error("Config error: {0}")]
    ConfigError(String),

    #[error("Operation failed: {0}")]
    OperationFailed(String),
}
