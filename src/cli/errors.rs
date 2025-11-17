use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, thiserror::Error)]
pub enum CliError {
    #[error("Parse Error: {0}")]
    ParseError(String),
    #[error("Command Error: {0}")]
    CommandError(String),
}
