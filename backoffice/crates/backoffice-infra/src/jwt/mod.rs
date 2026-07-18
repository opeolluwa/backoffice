use jsonwebtoken::{Header, decode, encode};
use serde::{Deserialize, Serialize};

use backoffice_domain::{
    dto::TokenClaims,
    ports::token_service::TokenService,
};
use backoffice_domain::errors::auth_service_error::AuthenticationServiceError;
use backoffice_domain::shared::extract_env::extract_env;

pub struct JwtTokenService;

impl JwtTokenService {
    pub fn new() -> Self {
        Self
    }
}

#[derive(Serialize, Deserialize)]
struct JwtClaim {
    pub email: String,
    pub identifier: String,
    pub iat: i64,
    pub exp: i64,
}

impl TokenService for JwtTokenService {
    fn generate_token(
        &self,
        claims: &TokenClaims,
        validity_secs: u64,
    ) -> Result<String, AuthenticationServiceError> {
        let now = chrono::Utc::now().timestamp();
        let claim = JwtClaim {
            email: claims.email.clone(),
            identifier: claims.identifier.clone(),
            iat: now,
            exp: now + validity_secs as i64,
        };

        let secret =
            extract_env::<String>("JWT_SIGNING_KEY").map_err(AuthenticationServiceError::from)?;

        let encoding_key =
            jsonwebtoken::EncodingKey::from_secret(secret.as_bytes());
        let token = encode(&Header::default(), &claim, &encoding_key)
            .map_err(AuthenticationServiceError::from)?;

        Ok(token)
    }

    fn validate_token(
        &self,
        token: &str,
    ) -> Result<TokenClaims, AuthenticationServiceError> {
        let secret =
            extract_env::<String>("JWT_SIGNING_KEY").map_err(AuthenticationServiceError::from)?;

        let decoding_key =
            jsonwebtoken::DecodingKey::from_secret(secret.as_bytes());

        let token_data = decode::<JwtClaim>(
            token,
            &decoding_key,
            &jsonwebtoken::Validation::default(),
        )
        .map_err(|_| AuthenticationServiceError::InvalidToken)?;

        Ok(TokenClaims {
            email: token_data.claims.email,
            identifier: token_data.claims.identifier,
        })
    }
}
