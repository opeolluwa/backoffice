#[tokio::main]
async fn main() {
    if let Err(err) = backoffice_console_lib::http::run().await {
        eprintln!("Server failed: {err}");
        std::process::exit(1);
    }
}
