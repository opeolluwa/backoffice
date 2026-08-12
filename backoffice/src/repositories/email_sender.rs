use crate::errors::email_service_error::EmailServiceError;

pub struct EmailMessage {
    pub from_address: String,
    pub from_name: String,
    pub to_address: String,
    pub to_name: String,
    pub subject: String,
    pub html_body: String,
}

#[cfg_attr(test, mockall::automock)]
pub trait EmailSender {
    fn send_email(&self, message: EmailMessage) -> Result<(), EmailServiceError>;
}
