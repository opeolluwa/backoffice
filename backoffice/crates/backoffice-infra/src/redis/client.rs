use backoffice_config::env::AppConfig;
use secrecy::ExposeSecret;

use crate::redis::error::RedisClientError;

pub struct RedisClient {
    connection: redis::Connection,
}

impl RedisClient {
    pub fn new(config: &AppConfig) -> Result<Self, RedisClientError> {
        let client = redis::Client::open(config.redis_url.expose_secret())?;
        let connection = client.get_connection()?;
        Ok(Self { connection })
    }
}
