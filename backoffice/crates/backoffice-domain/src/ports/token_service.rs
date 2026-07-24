use crate::dto::TokenClaims;
use crate::errors::auth_service_error::AuthenticationServiceError;

#[cfg_attr(test, mockall::automock)]
pub trait TokenService: Send + Sync {
    fn generate_token(
        &self,
        claims: &TokenClaims,
        validity_secs: u64,
    ) -> Result<String, AuthenticationServiceError>;

    fn validate_token(&self, token: &str) -> Result<TokenClaims, AuthenticationServiceError>;
}

/// Extended trait for token validation that exposes expiry information.
pub trait TokenServiceExt: TokenService {
    fn validate_token_with_expiry(
        &self,
        token: &str,
    ) -> Result<(TokenClaims, i64), AuthenticationServiceError>;
}
