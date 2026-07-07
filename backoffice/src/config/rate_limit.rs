pub fn init_rate_limit(config: &AppConfig) -> Option<RateLimitLayer> {
    if config.environment == config::env::Environment::Production {
        Some(RateLimitLayer::new(
            config.rate_limit_requests,
            config.rate_limit_window,
        ))
    } else {
        None
    }
}
