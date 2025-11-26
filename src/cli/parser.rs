use clap::ArgMatches;

pub fn parse_commands(matches: ArgMatches) {
    match matches.subcommand() {
        _ => {
            // LogMessage::error("Invalid command");
            std::process::exit(1)
        }
    }
}
