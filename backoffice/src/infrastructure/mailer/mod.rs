pub mod auto_respond;
pub mod invitation_accepted;
pub mod password_reset;
pub mod zepto_mailer;

use crate::domain::{dto::EmailMessage, ports::email_sender::EmailSender};
use crate::errors::email_service_error::EmailServiceError;
use zepto_mailer::{EmailRequestBuilder, ZeptoMail};

impl EmailSender for ZeptoMail {
    async fn send_email(&self, message: EmailMessage) -> Result<(), EmailServiceError> {
        let email_request = EmailRequestBuilder::new()
            .from(&message.from_address, &message.from_name)
            .to(&message.to_address, &message.to_name)
            .subject(&message.subject)
            .html_body(&message.html_body)
            .build();

        self.send_email(email_request)
            .await
            .map_err(|e| EmailServiceError::OperationFailed(e.to_string()))
    }
}
