use clap::ArgMatches;

pub fn parse_commands(matches: ArgMatches) {
    match matches.subcommand() {
        Some(("create-user", sub_matches)) => {
            let first_name = sub_matches.get_one::<String>("first_name").unwrap();
            let last_name = sub_matches.get_one::<String>("last_name").unwrap();
            let email = sub_matches.get_one::<String>("email").unwrap();
            let _password = sub_matches.get_one::<String>("password").unwrap();

            println!(
                "Creating user: {} {} ({})",
                first_name, last_name, email
            );

            // TODO: Call the user service to create the user with the provided details
        }
        _ => {
            // LogMessage::error("Invalid command");
            std::process::exit(1)
        }
    }
}
