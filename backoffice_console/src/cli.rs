use clap::{Parser, Subcommand};

use crate::commands;
use crate::errors::CliError;

#[derive(Parser)]
#[command(
    name = "backoffice console",
    version = "0.1.0",
    about = "Code generator for backoffice application",
    subcommand_required = true,
    arg_required_else_help = true
)]
struct BackofficeCli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Generate a new module (backend + frontend + migration)
    CreateModule,
}

pub fn run() -> Result<(), CliError> {
    let cli = BackofficeCli::parse();

    match cli.command {
        Commands::CreateModule => commands::create_module::run(),
    }
}
