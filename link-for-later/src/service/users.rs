use axum::async_trait;

use crate::{
    repository,
    service::Users as UsersService,
    types::{entity::UserInfo, Result},
};

pub struct ServiceProvider {}

#[async_trait]
impl UsersService for ServiceProvider {
    async fn add(
        &self,
        users_repo: Box<repository::DynUsers>,
        info: &UserInfo,
    ) -> Result<UserInfo> {
        users_repo.add(info).await
    }

    async fn find(&self, users_repo: Box<repository::DynUsers>, id: &str) -> Result<UserInfo> {
        users_repo.find(id).await
    }
}
