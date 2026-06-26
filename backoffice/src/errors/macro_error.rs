#[derive(Debug, thiserror::Error)]
pub enum MacroError {
    #[error("Failed to generate SQL")]
    SQLGenerationFailed,
    #[error("Failed to generate SQL")]
    InvalidField,
}
