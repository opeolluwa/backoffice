fn main() {
    if let Err(e) = backoffice_console_lib::cli::run() {
        eprintln!("Error: {e}");
        std::process::exit(1);
    }
}
