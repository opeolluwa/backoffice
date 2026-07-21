use lettre::{
    Message, SmtpTransport, Transport,
    message::{MultiPart, SinglePart, header},
    transport::smtp::authentication::Credentials,
};

use backoffice_domain::{
    errors::email_service_error::EmailServiceError,
    ports::email_sender::{EmailMessage, EmailSender},
};

#[derive(Clone)]
pub struct SmtpEmailSender {
    mailer: SmtpTransport,
}

impl SmtpEmailSender {
    pub fn new(
        host: &str,
        port: u16,
        username: &str,
        password: &str,
    ) -> Result<Self, EmailServiceError> {
        let creds = Credentials::new(username.to_string(), password.to_string());

        let mailer = SmtpTransport::starttls_relay(host)
            .unwrap()
            .credentials(creds)
            .build();

        Ok(Self { mailer })
    }
}

impl EmailSender for SmtpEmailSender {
    fn send_email(&self, message: EmailMessage) -> Result<(), EmailServiceError> {
        let email = Message::builder()
            .from(
                format!("{} <{}>", message.from_name, message.from_address)
                    .parse()
                    .unwrap(),
            )
            .to(format!("{} <{}>", message.to_name, message.to_address)
                .parse()
                .unwrap())
            .subject(message.subject)
            .multipart(
                MultiPart::alternative().singlepart(
                    SinglePart::builder()
                        .header(header::ContentType::TEXT_HTML)
                        .body(String::from(message.html_body)),
                ),
            )
            .unwrap();

        self.mailer
            .send(&email)
            .map_err(|err| EmailServiceError::DeliveryError(err.to_string()))?;

        Ok(())
    }
}
