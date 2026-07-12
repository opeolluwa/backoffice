use clap::Command;

use crate::cli::{errors::CliError, parser::parse_commands};

pub async fn run() -> Result<(), CliError> {
    let matches = Command::new("backoffice CLI")
        .version("0.1.0")
        .about("Command line interface for backoffice application")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(Command::new("init").about("Initialize the backoffice application"))
        .subcommand(
            Command::new("create-user")
                .about("create a new user account")
                .arg(
                    clap::Arg::new("first_name")
                        .long("first-name")
                        .required(true)
                        .help("User's first name"),
                )
                .arg(
                    clap::Arg::new("last_name")
                        .long("last-name")
                        .required(true)
                        .help("User's last name"),
                )
                .arg(
                    clap::Arg::new("email")
                        .long("email")
                        .required(true)
                        .help("User's email address"),
                )
                .arg(
                    clap::Arg::new("password")
                        .long("password")
                        .required(true)
                        .help("User's password"),
                ),
        )
        .get_matches();

    parse_commands(matches);

    Ok(())
}
