use lettre::{
    Message, SmtpTransport, Transport,
    message::{MultiPart, SinglePart, header},
    transport::smtp::authentication::Credentials,
};

use crate::{
    errors::email_service_error::EmailServiceError,
    repositories::email_sender::{EmailMessage, EmailSender},
};

#[derive(Clone)]
pub struct SmtpEmailSender {
    mailer: SmtpTransport,
}

impl SmtpEmailSender {
    pub fn new(
        host: &str,
        _port: u16,
        username: &str,
        password: &str,
    ) -> Result<Self, EmailServiceError> {
        let creds = Credentials::new(username.to_string(), password.to_string());

        let mailer = SmtpTransport::starttls_relay(host)
            .map_err(|e| EmailServiceError::ProviderError(e.to_string()))?
            .credentials(creds)
            .build();

        Ok(Self { mailer })
    }
}

impl EmailSender for SmtpEmailSender {
    fn send_email(&self, message: EmailMessage) -> Result<(), EmailServiceError> {
        let from = format!("{} <{}>", message.from_name, message.from_address)
            .parse()
            .map_err(|e| {
                EmailServiceError::OperationFailed(format!("invalid from address: {e}"))
            })?;

        let to = format!("{} <{}>", message.to_name, message.to_address)
            .parse()
            .map_err(|e| EmailServiceError::OperationFailed(format!("invalid to address: {e}")))?;

        let email = Message::builder()
            .from(from)
            .to(to)
            .subject(message.subject)
            .multipart(
                MultiPart::alternative().singlepart(
                    SinglePart::builder()
                        .header(header::ContentType::TEXT_HTML)
                        .body(String::from(message.html_body)),
                ),
            )
            .map_err(|e| EmailServiceError::OperationFailed(e.to_string()))?;

        self.mailer
            .send(&email)
            .map_err(|err| EmailServiceError::DeliveryError(err.to_string()))?;

        Ok(())
    }
}
