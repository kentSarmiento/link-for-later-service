use axum::{async_trait, extract::FromRequestParts, http::request::Parts};
use axum_extra::{
    headers::{authorization::Bearer, Authorization},
    TypedHeader,
};
use jsonwebtoken::{decode, DecodingKey, Validation};

use crate::types::{auth::Claims, AppError};

#[async_trait]
impl<S> FromRequestParts<S> for Claims
where
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let TypedHeader(Authorization(bearer)) =
            TypedHeader::<Authorization<Bearer>>::from_request_parts(parts, state)
                .await
                .map_err(|_| AppError::AuthorizationError)?;

        let secret = std::env::var("JWT_SECRET").map_or_else(|_| String::new(), |secret| secret);
        let token_data = match decode::<Self>(
            bearer.token(),
            &DecodingKey::from_secret(secret.as_bytes()),
            &Validation::default(),
        ) {
            Ok(token) => token,
            Err(e) => {
                tracing::error!("Error: {}", e.to_string());
                return Err(AppError::AuthorizationError);
            }
        };

        Ok(token_data.claims)
    }
}
