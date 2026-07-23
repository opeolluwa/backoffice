use backoffice_config::env::AppConfig;
use secrecy::ExposeSecret;

use crate::redis::error::RedisClientError;

pub struct RedisClient {
    client: redis::Client,
}

impl RedisClient {
    pub fn new(config: &AppConfig) -> Result<Self, RedisClientError> {
        let client = redis::Client::open(config.redis_url.expose_secret())?;
        // Verify connection is available
        let mut conn = client.get_connection()?;
        redis::cmd("PING").execute(&mut conn)?;
        Ok(Self { client })
    }

    pub fn get_connection(&self) -> Result<redis::Connection, RedisClientError> {
        Ok(self.client.get_connection()?)
    }

    pub fn blacklist_token(&self, token: &str, expiry_secs: u64) -> Result<(), RedisClientError> {
        let mut conn = self.get_connection()?;
        let key = format!("blacklist:{}", token);
        redis::cmd("SETEX")
            .arg(&key)
            .arg(expiry_secs)
            .arg("1")
            .execute(&mut conn);
        Ok(())
    }

    pub fn is_token_blacklisted(&self, token: &str) -> Result<bool, RedisClientError> {
        let mut conn = self.get_connection()?;
        let key = format!("blacklist:{}", token);
        let exists: bool = redis::cmd("EXISTS").arg(&key).query(&mut conn)?;
        Ok(exists)
    }
}
