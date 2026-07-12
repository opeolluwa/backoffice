use crate::errors::email_service_error::EmailServiceError;
use serde::Serialize;

#[derive(Serialize)]
pub struct EmailPayload<T>
where
    T: serde::Serialize,
{
    pub to: String,
    pub subject: String,
    pub body: String,
    pub data: T,
}

pub trait EmailService {
    fn send_email<T>(&self, payload: EmailPayload<T>) -> Result<(), EmailServiceError>
    where
        T: serde::Serialize;
}
