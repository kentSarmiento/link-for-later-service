use axum::{async_trait, extract::FromRequestParts, http::request::Parts};
use axum_extra::{
    headers::{authorization::Bearer, Authorization},
    TypedHeader,
};
use jsonwebtoken::{decode, DecodingKey, Validation};

use crate::{dto::Claims, types::AppError};

const JWT_SECRET_KEY: &str = "JWT_SECRET";

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
                .map_err(|_| {
                    AppError::Authorization(String::from("Authorization token not found"))
                })?;

        let secret =
            std::env::var(JWT_SECRET_KEY).map_or_else(|_| String::default(), |secret| secret);
        let token_data = decode::<Self>(
            bearer.token(),
            &DecodingKey::from_secret(secret.as_bytes()),
            &Validation::default(),
        )
        .map_err(|e| AppError::Authorization(format!("decode() {e:?}")))?;

        Ok(token_data.claims)
    }
}
