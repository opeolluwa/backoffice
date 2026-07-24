use backoffice_config::env::AppConfig;
use secrecy::ExposeSecret;

use crate::redis::error::RedisClientError;

#[derive(Clone)]
pub struct RedisClient {
    client: redis::Client,
}

impl RedisClient {
    pub fn new(config: &AppConfig) -> Result<Self, RedisClientError> {
        let client = redis::Client::open(config.redis_url.expose_secret())?;
        let mut conn = client.get_connection()?;
        redis::cmd("PING").exec(&mut conn)?;
        Ok(Self { client })
    }

    pub fn get_connection(&self) -> Result<redis::Connection, RedisClientError> {
        Ok(self.client.get_connection()?)
    }

    pub fn blacklist_token(&self, token: &str, expiry_secs: u64) -> Result<(), RedisClientError> {
        let mut conn = self.get_connection()?;
        let key = format!("blacklist:{}", token);
        let _ = redis::cmd("SETEX")
            .arg(&key)
            .arg(expiry_secs)
            .arg("1")
            .exec(&mut conn);
        Ok(())
    }

    pub fn is_token_blacklisted(&self, token: &str) -> Result<bool, RedisClientError> {
        let mut conn = self.get_connection()?;
        let key = format!("blacklist:{}", token);
        let exists: bool = redis::cmd("EXISTS").arg(&key).query(&mut conn)?;
        Ok(exists)
    }

    pub fn store_refresh_token(
        &self,
        token: &str,
        user_identifier: &str,
        expiry_secs: u64,
    ) -> Result<(), RedisClientError> {
        let mut conn = self.get_connection()?;
        let key = format!("refresh:{}:{}", user_identifier, token);
        let _ = redis::cmd("SETEX")
            .arg(&key)
            .arg(expiry_secs)
            .arg("1")
            .exec(&mut conn);
        Ok(())
    }

    pub fn is_refresh_token_valid(
        &self,
        token: &str,
        user_identifier: &str,
    ) -> Result<bool, RedisClientError> {
        let mut conn = self.get_connection()?;
        let key = format!("refresh:{}:{}", user_identifier, token);
        let exists: bool = redis::cmd("EXISTS").arg(&key).query(&mut conn)?;
        Ok(exists)
    }

    pub fn revoke_refresh_tokens_for_user(
        &self,
        user_identifier: &str,
    ) -> Result<(), RedisClientError> {
        let mut conn = self.get_connection()?;
        let pattern = format!("refresh:{}:*", user_identifier);
        let keys: Vec<String> = redis::cmd("KEYS")
            .arg(&pattern)
            .query(&mut conn)?;
        for key in &keys {
            let _ = redis::cmd("DEL").arg(key).exec(&mut conn);
        }
        Ok(())
    }
}
