use clap::Command;

use crate::cli::{errors::CliError, parser::parse_commands};

pub async fn run() -> Result<(), CliError> {
    let matches = Command::new("backoffice CLI")
        .version("0.1.0")
        .about("Command line interface for backoffice application")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(Command::new("init").about("Initialize the backoffice application"))
        .subcommand(Command::new("create-user").about("create a new user account"))
        .get_matches();

    parse_commands(matches);

    Ok(())
}
