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
use jsonwebtoken::{Validation, decode};

use crate::http::dto::jwt::{Claims, Keys};
use backoffice_domain::errors::auth_service_error::AuthenticationServiceError;
use backoffice_domain::shared::extract_env::extract_env;

impl<S> FromRequestParts<S> for Claims
where
    S: Send + Sync,
{
    type Rejection = AuthenticationServiceError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let secret =
            extract_env::<String>("JWT_SIGNING_KEY").map_err(AuthenticationServiceError::from)?;

        let decoding_key = Keys::new(secret.as_bytes()).decoding;
        // Extract the token from the authorization header
        let TypedHeader(Authorization(bearer)) = parts
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await
            .map_err(|_| AuthenticationServiceError::MissingCredentials)?;
        // Decode the user data
        let token_data = decode::<Claims>(bearer.token(), &decoding_key, &Validation::default())
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
    // Extract the token from the authorization header
    let TypedHeader(Authorization(bearer)) = parts
        .extract::<TypedHeader<Authorization<Bearer>>>()
        .await
        .map_err(|_| AuthenticationServiceError::MissingCredentials)?;

    // Decode the user data
    let token_data = decode::<Claims>(bearer.token(), &decoding_key, &Validation::default())
        .map_err(|_| AuthenticationServiceError::InvalidToken)?;

    request = Request::from_parts(parts, body);

    request.extensions_mut().insert(token_data.claims);

    Ok(next.run(request).await)
}
