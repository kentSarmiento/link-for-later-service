use axum::async_trait;
use chrono::{DateTime, Duration, Utc};
use jsonwebtoken::{encode, EncodingKey, Header};
use std::convert::TryInto;

use crate::{
    repository,
    service::Users as UsersService,
    types::{
        auth::{Claims, Token},
        dto::UserQueryBuilder,
        entity::{UserInfo, UserInfoBuilder},
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
        let user_query = UserQueryBuilder::new(user_info.email()).build();
        let user_info = match users_repo.search(&user_query).await {
            Ok(_) => return Err(AppError::UserAlreadyExists),
            Err(AppError::UserNotFound) => user_info.clone(),
            Err(e) => return Err(e),
        };

        let now = Utc::now().to_rfc3339();
        // TODO: secure password
        let registered_user_info = UserInfoBuilder::from(user_info.clone())
            .created_at(&now)
            .updated_at(&now)
            .verified(true)
            .build();

        users_repo.create(&registered_user_info).await
    }

    async fn login(
        &self,
        users_repo: Box<repository::DynUsers>,
        user_info: &UserInfo,
    ) -> Result<Token> {
        let user_query = UserQueryBuilder::new(user_info.email()).build();
        let retrieved_user_info = users_repo.search(&user_query).await?;

        if retrieved_user_info.password() != user_info.password() {
            tracing::info!("invalid password for user {}", &user_info.email());
            return Err(AppError::IncorrectPassword);
        }

        let timestamp = |timestamp: DateTime<Utc>| -> Result<usize> {
            let timestamp: usize = timestamp
                .timestamp()
                .try_into()
                .map_err(|_| AppError::ServerError)?;
            Ok(timestamp)
        };

        let now = Utc::now();
        let claims = Claims::new(
            retrieved_user_info.email(),
            timestamp(now)?,
            timestamp(now + Duration::minutes(60))?,
        );

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
