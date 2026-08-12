use jsonwebtoken::{Algorithm, Header, decode, encode};
use serde::{Deserialize, Serialize};

use crate::errors::auth_service_error::AuthenticationServiceError;
use crate::utils::extract_env::extract_env;
use crate::{dto::TokenClaims, repositories::token_service::TokenService};

#[derive(Clone)]
pub struct JwtTokenService;

impl JwtTokenService {
    pub fn new() -> Self {
        Self
    }
}

fn jwt_validation() -> jsonwebtoken::Validation {
    let mut validation = jsonwebtoken::Validation::new(Algorithm::HS256);
    validation.set_required_spec_claims(&["exp", "iat"]);
    validation
}

#[derive(Serialize, Deserialize)]
struct JwtClaim {
    pub email: String,
    pub identifier: String,
    #[serde(default = "default_token_type")]
    pub token_type: String,
    pub iat: i64,
    pub exp: i64,
}

fn default_token_type() -> String {
    "access".to_string()
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
            token_type: claims.token_type.clone(),
            iat: now,
            exp: now + validity_secs as i64,
        };

        let secret =
            extract_env::<String>("JWT_SIGNING_KEY").map_err(AuthenticationServiceError::from)?;

        let encoding_key = jsonwebtoken::EncodingKey::from_secret(secret.as_bytes());
        let token = encode(&Header::default(), &claim, &encoding_key)
            .map_err(AuthenticationServiceError::from)?;

        Ok(token)
    }

    fn validate_token(&self, token: &str) -> Result<TokenClaims, AuthenticationServiceError> {
        let secret =
            extract_env::<String>("JWT_SIGNING_KEY").map_err(AuthenticationServiceError::from)?;

        let decoding_key = jsonwebtoken::DecodingKey::from_secret(secret.as_bytes());

        let token_data = decode::<JwtClaim>(token, &decoding_key, &jwt_validation())
            .map_err(|_| AuthenticationServiceError::InvalidToken)?;

        Ok(TokenClaims {
            email: token_data.claims.email,
            identifier: token_data.claims.identifier,
            token_type: token_data.claims.token_type,
        })
    }
}
