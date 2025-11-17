use backoffice_lib::cli::{self, errors::CliError};

#[tokio::main]
async fn main() -> Result<(), CliError> {
    cli::app::run().await?;
    Ok(())
}
