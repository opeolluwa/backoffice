use backoffice_lib::errors::app_error::AppError;

#[tokio::main]
async fn main() -> Result<(), AppError> {
    backoffice_lib::app::run().await
}
