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
        hash(raw_password.trim(), DEFAULT_COST)
            .map_err(|err| ServiceError::OperationFailed(err.to_string()))
    }
    fn validate_password(&self, password: &str, hash: &str) -> Result<bool, ServiceError> {
        verify(password, hash).map_err(|err| ServiceError::OperationFailed(err.to_string()))
    }
}
