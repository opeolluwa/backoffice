pub struct AppLogger {}

pub fn init_tracing() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .with_target(false)
        .compact()
        .init();
    tracing::info!("Logger initialized");
}
