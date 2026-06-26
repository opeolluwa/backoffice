pub mod error;

pub use backoffice_email_client::auto_respond::AutoRespondTemplate;
pub use backoffice_email_client::password_reset::PasswordResetTemplate;
pub use backoffice_email_client::zepto_mailer::{EmailRequestBuilder, ZeptoMail};
