use axum::{
    RequestPartsExt,
    extract::{FromRequestParts, Request},
    http::request::Parts,
    middleware::Next,
    response::Response,
};
use axum_extra::{
    TypedHeader,
    headers::{Authorization, authorization::Bearer},
};
use jsonwebtoken::{Algorithm, Validation, decode};

use backoffice_domain::errors::auth_service_error::AuthenticationServiceError;
use backoffice_domain::shared::extract_env::extract_env;

use crate::http::dto::jwt::{Claims, Keys};
use backoffice_infra::redis::client::RedisClient;

fn jwt_validation() -> Validation {
    let mut validation = Validation::new(Algorithm::HS256);
    validation.set_required_spec_claims(&["exp", "iat"]);
    validation
}

impl<S> FromRequestParts<S> for Claims
where
    S: Send + Sync,
{
    type Rejection = AuthenticationServiceError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let secret =
            extract_env::<String>("JWT_SIGNING_KEY").map_err(AuthenticationServiceError::from)?;

        let decoding_key = Keys::new(secret.as_bytes()).decoding;
        let TypedHeader(Authorization(bearer)) = parts
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await
            .map_err(|_| AuthenticationServiceError::MissingCredentials)?;
        let token_data = decode::<Claims>(bearer.token(), &decoding_key, &jwt_validation())
            .map_err(|_| AuthenticationServiceError::InvalidToken)?;

        Ok(token_data.claims)
    }
}

pub async fn authenticate(
    mut request: Request,
    next: Next,
) -> Result<Response, AuthenticationServiceError> {
    let (mut parts, body) = request.into_parts();

    let secret =
        extract_env::<String>("JWT_SIGNING_KEY").map_err(AuthenticationServiceError::from)?;

    let decoding_key = Keys::new(secret.as_bytes()).decoding;
    let TypedHeader(Authorization(bearer)) = parts
        .extract::<TypedHeader<Authorization<Bearer>>>()
        .await
        .map_err(|_| AuthenticationServiceError::MissingCredentials)?;

    let token = bearer.token();

    if let Some(redis) = parts.extensions.get::<RedisClient>() {
        let is_blacklisted = redis
            .is_token_blacklisted(token)
            .map_err(|e| AuthenticationServiceError::OperationFailed(e.to_string()))?;

        if is_blacklisted {
            return Err(AuthenticationServiceError::InvalidToken);
        }
    }

    let token_data = decode::<Claims>(token, &decoding_key, &jwt_validation())
        .map_err(|_| AuthenticationServiceError::InvalidToken)?;

    request = Request::from_parts(parts, body);

    request.extensions_mut().insert(token_data.claims);
    request.extensions_mut().insert(RawToken(token.to_string()));

    Ok(next.run(request).await)
}

#[derive(Clone)]
pub struct RawToken(pub String);
