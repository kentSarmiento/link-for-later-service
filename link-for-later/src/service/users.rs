use axum::async_trait;
use chrono::Utc;
use jsonwebtoken::{encode, EncodingKey, Header};

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
        secret_key: &str,
        user_info: &UserInfo,
    ) -> Result<Token> {
        let retrieved_user_info = users_repo.find_by_user(&user_info.email).await?;

        if retrieved_user_info.password != user_info.password {
            tracing::error!("Error: invalid password for user {}", &user_info.email);
            return Err(AppError::InvalidPassword);
        }

        let claims = Claims::new(&retrieved_user_info.email);
        let Ok(token) = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(secret_key.as_ref()),
        ) else {
            tracing::error!("Error: cannot generate token");
            return Err(AppError::ServerError);
        };

        Ok(Token::new(&token))
    }
}
