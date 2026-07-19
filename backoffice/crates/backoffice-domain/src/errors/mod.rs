pub mod app_error;
pub mod auth_service_error;
pub mod authentication_error;
pub mod database_error;
pub mod email_service_error;
pub mod filesystem_error;
pub mod imagekit_error;
pub mod macro_error;
pub mod service_error;

#[cfg(feature = "http")]
pub mod api_response;
