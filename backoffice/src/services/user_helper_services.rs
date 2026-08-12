use bcrypt::{DEFAULT_COST, hash, verify};

use crate::errors::service_error::ServiceError;

#[derive(Clone)]
pub struct UserHelperService {}

impl UserHelperService {
    pub fn init() -> Self {
        Self {}
    }
}

pub trait UserHelperServiceTrait {
    fn hash_password(&self, raw_password: &str) -> Result<String, ServiceError>;
    fn validate_password(&self, raw_password: &str, hash: &str) -> Result<bool, ServiceError>;
}

impl UserHelperServiceTrait for UserHelperService {
    fn hash_password(&self, raw_password: &str) -> Result<String, ServiceError> {
        Ok(hash(raw_password.trim(), DEFAULT_COST)?)
    }
    fn validate_password(&self, password: &str, hash: &str) -> Result<bool, ServiceError> {
        Ok(verify(password, hash)?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hash_password_returns_bcrypt_hash() {
        let service = UserHelperService::init();
        let hashed = service.hash_password("mypassword").unwrap();
        assert!(hashed.starts_with("$2"));
        assert!(hashed.len() > 20);
    }

    #[test]
    fn hash_password_trims_whitespace() {
        let service = UserHelperService::init();
        let h1 = service.hash_password("password").unwrap();
        let h2 = service.hash_password("  password  ").unwrap();
        assert!(service.validate_password("password", &h1).unwrap());
        assert!(service.validate_password("password", &h2).unwrap());
    }

    #[test]
    fn validate_password_correct() {
        let service = UserHelperService::init();
        let hashed = service.hash_password("secret123").unwrap();
        assert!(service.validate_password("secret123", &hashed).unwrap());
    }

    #[test]
    fn validate_password_incorrect() {
        let service = UserHelperService::init();
        let hashed = service.hash_password("secret123").unwrap();
        assert!(!service.validate_password("wrongpassword", &hashed).unwrap());
    }
}
