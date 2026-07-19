fn main() {
    if let Err(e) = backoffice_cli::app::run() {
        eprintln!("Error: {e}");
        std::process::exit(1);
    }
}
