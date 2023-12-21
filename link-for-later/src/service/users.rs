use axum::async_trait;
use chrono::Utc;

use crate::{
    repository,
    service::Users as UsersService,
    types::{entity::UserInfo, AppError, Result},
};

pub struct ServiceProvider {}

#[async_trait]
impl UsersService for ServiceProvider {
    async fn register(
        &self,
        users_repo: Box<repository::DynUsers>,
        user_info: &UserInfo,
    ) -> Result<UserInfo> {
        if users_repo.find_by_user(&user_info.email).await.is_ok() {
            return Err(AppError::UserAlreadyExists);
        }

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
    ) -> Result<UserInfo> {
        if users_repo.find_by_user(&user_info.email).await.is_err() {
            return Err(AppError::UserNotFound);
        }

        // TODO: login process, return token
        Ok(user_info.clone())
    }
}
