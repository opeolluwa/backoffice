use crate::dto::EmailMessage;
use crate::errors::email_service_error::EmailServiceError;

pub trait EmailSender: Send + Sync {
    fn send_email(
        &self,
        message: EmailMessage,
    ) -> impl std::future::Future<Output = Result<(), EmailServiceError>> + Send;
}
