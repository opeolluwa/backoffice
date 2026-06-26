#[derive(Debug, thiserror::Error)]
pub enum MailerError {
    #[error("failed to send email: {0}")]
    SendFailed(String),
    #[error("invalid recipient: {0}")]
    InvalidRecipient(String),
    #[error("template rendering failed: {0}")]
    TemplateFailed(String),
    #[error("mailer configuration error: {0}")]
    ConfigurationError(String),
}
