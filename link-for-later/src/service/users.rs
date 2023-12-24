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

const JWT_SECRET_KEY: &str = "JWT_SECRET";

pub struct ServiceProvider {}

#[async_trait]
impl UsersService for ServiceProvider {
    async fn register(
        &self,
        users_repo: Box<repository::DynUsers>,
        user_info: &UserInfo,
    ) -> Result<UserInfo> {
        let user_query = UserQueryBuilder::new(user_info.email()).build();
        let user_info = match users_repo.get(&user_query).await {
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
        let retrieved_user_info = users_repo.get(&user_query).await?;

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

        let secret =
            std::env::var(JWT_SECRET_KEY).map_or_else(|_| String::default(), |secret| secret);
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

#[cfg(test)]
mod tests {

    use std::sync::Arc;

    use crate::{repository::MockUsers as MockUsersRepo, types::AppError};

    use super::*;

    #[tokio::test]
    async fn test_register_user() {
        let repo_query = UserQueryBuilder::new("user@test.com").build();
        let user_to_register = UserInfoBuilder::new("user@test.com", "test").build();
        let registered_user = UserInfoBuilder::new("user@test.com", "test").build();
        let request_item = user_to_register.clone();
        let response_item = registered_user.clone();

        let mut mock_users_repo = MockUsersRepo::new();
        mock_users_repo
            .expect_get()
            .withf(move |query| query == &repo_query)
            .times(1)
            .returning(|_| Err(AppError::UserNotFound));
        mock_users_repo
            .expect_create()
            //.withf(move |user| user == &user_to_register)
            .times(1)
            .returning(move |_| Ok(registered_user.clone()));

        let users_service = ServiceProvider {};
        let response = users_service
            .register(Box::new(Arc::new(mock_users_repo)), &request_item)
            .await;

        assert!(response.is_ok());
        assert_eq!(response.unwrap(), response_item);
    }

    #[tokio::test]
    async fn test_register_user_already_registered() {
        let repo_query = UserQueryBuilder::new("user@test.com").build();
        let user_to_register = UserInfoBuilder::new("user@test.com", "test").build();
        let registered_user = UserInfoBuilder::new("user@test.com", "test").build();
        let request_item = user_to_register.clone();

        let mut mock_users_repo = MockUsersRepo::new();
        mock_users_repo
            .expect_get()
            .withf(move |query| query == &repo_query)
            .times(1)
            .returning(move |_| Ok(registered_user.clone()));
        mock_users_repo.expect_create().times(0);

        let users_service = ServiceProvider {};
        let response = users_service
            .register(Box::new(Arc::new(mock_users_repo)), &request_item)
            .await;

        assert_eq!(response, Err(AppError::UserAlreadyExists));
    }

    #[tokio::test]
    async fn test_register_user_get_repo_error() {
        let repo_query = UserQueryBuilder::new("user@test.com").build();
        let user_to_register = UserInfoBuilder::new("user@test.com", "test").build();
        let request_item = user_to_register.clone();

        let mut mock_users_repo = MockUsersRepo::new();
        mock_users_repo
            .expect_get()
            .withf(move |query| query == &repo_query)
            .times(1)
            .returning(|_| Err(AppError::ServerError));
        mock_users_repo.expect_create().times(0);

        let users_service = ServiceProvider {};
        let response = users_service
            .register(Box::new(Arc::new(mock_users_repo)), &request_item)
            .await;

        assert_eq!(response, Err(AppError::ServerError));
    }

    #[tokio::test]
    async fn test_register_user_create_repo_error() {
        let repo_query = UserQueryBuilder::new("user@test.com").build();
        let user_to_register = UserInfoBuilder::new("user@test.com", "test").build();
        let request_item = user_to_register.clone();

        let mut mock_users_repo = MockUsersRepo::new();
        mock_users_repo
            .expect_get()
            .withf(move |query| query == &repo_query)
            .times(1)
            .returning(|_| Err(AppError::UserNotFound));
        mock_users_repo
            .expect_create()
            //.withf(move |user| user == &user_to_register)
            .times(1)
            .returning(move |_| Err(AppError::ServerError));

        let users_service = ServiceProvider {};
        let response = users_service
            .register(Box::new(Arc::new(mock_users_repo)), &request_item)
            .await;

        assert_eq!(response, Err(AppError::ServerError));
    }

    #[tokio::test]
    async fn test_login_user() {
        let repo_query = UserQueryBuilder::new("user@test.com").build();
        let user_to_login = UserInfoBuilder::new("user@test.com", "test").build();
        let registered_user = UserInfoBuilder::new("user@test.com", "test").build();
        let request_item = user_to_login.clone();

        let mut mock_users_repo = MockUsersRepo::new();
        mock_users_repo
            .expect_get()
            .withf(move |query| query == &repo_query)
            .times(1)
            .returning(move |_| Ok(registered_user.clone()));

        let users_service = ServiceProvider {};
        let response = users_service
            .login(Box::new(Arc::new(mock_users_repo)), &request_item)
            .await;

        assert!(response.is_ok());
    }

    #[tokio::test]
    async fn test_login_user_not_registered() {
        let repo_query = UserQueryBuilder::new("user@test.com").build();
        let user_to_login = UserInfoBuilder::new("user@test.com", "test").build();
        let request_item = user_to_login.clone();

        let mut mock_users_repo = MockUsersRepo::new();
        mock_users_repo
            .expect_get()
            .withf(move |query| query == &repo_query)
            .times(1)
            .returning(move |_| Err(AppError::UserNotFound));

        let users_service = ServiceProvider {};
        let response = users_service
            .login(Box::new(Arc::new(mock_users_repo)), &request_item)
            .await;

        assert_eq!(response, Err(AppError::UserNotFound));
    }

    #[tokio::test]
    async fn test_login_user_incorrect_password() {
        let repo_query = UserQueryBuilder::new("user@test.com").build();
        let user_to_login = UserInfoBuilder::new("user@test.com", "incorrect").build();
        let registered_user = UserInfoBuilder::new("user@test.com", "test").build();
        let request_item = user_to_login.clone();

        let mut mock_users_repo = MockUsersRepo::new();
        mock_users_repo
            .expect_get()
            .withf(move |query| query == &repo_query)
            .times(1)
            .returning(move |_| Ok(registered_user.clone()));

        let users_service = ServiceProvider {};
        let response = users_service
            .login(Box::new(Arc::new(mock_users_repo)), &request_item)
            .await;

        assert_eq!(response, Err(AppError::IncorrectPassword));
    }
}
