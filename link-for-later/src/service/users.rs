use axum::async_trait;
use chrono::{Duration, Utc};
use jsonwebtoken::{encode, EncodingKey, Header};
use std::convert::TryInto;

use crate::{
    repository,
    service::Users as UsersService,
    types::{
        auth::{Claims, Token},
        entity::UserInfo,
        AppError, Result,
    },
};

pub struct ServiceProvider {}

#[async_trait]
impl UsersService for ServiceProvider {
    async fn register(
        &self,
        users_repo: Box<repository::DynUsers>,
        user_info: &UserInfo,
    ) -> Result<UserInfo> {
        let user_info = match users_repo.find_by_user(&user_info.email).await {
            Ok(_) => return Err(AppError::UserAlreadyExists),
            Err(AppError::UserNotFound) => user_info.clone(),
            Err(e) => return Err(e),
        };

        let now = Utc::now().to_rfc3339();
        let registered_user_info = user_info.created_at(&now).updated_at(&now);

        // TODO: verification process (e.g. valid email)
        let verified_user_info = registered_user_info.verified(true);

        // TODO: secure password
        users_repo.add(&verified_user_info).await
    }

    async fn login(
        &self,
        users_repo: Box<repository::DynUsers>,
        user_info: &UserInfo,
    ) -> Result<Token> {
        let retrieved_user_info = users_repo.find_by_user(&user_info.email).await?;

        if retrieved_user_info.password != user_info.password {
            tracing::info!("invalid password for user {}", &user_info.email);
            return Err(AppError::InvalidPassword);
        }

        let expiration = Utc::now() + Duration::minutes(60);
        let expiration: usize = expiration
            .timestamp()
            .try_into()
            .map_err(|_| AppError::ServerError)?;

        let claims = Claims::new(&retrieved_user_info.email, expiration);

        let secret = std::env::var("JWT_SECRET").map_or_else(|_| String::new(), |secret| secret);
        let token = match encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(secret.as_bytes()),
        ) {
            Ok(token) => token,
            Err(e) => {
                tracing::error!("Error: {}", e.to_string());
                return Err(AppError::ServerError);
            }
        };

        Ok(Token::new(&token))
    }
}
