use axum::async_trait;

use crate::{
    service::Users as UsersService,
    state::AppState,
    types::{entity::UserInfo, Result},
};

pub struct ServiceProvider {}

#[async_trait]
impl UsersService for ServiceProvider {
    async fn add<'a>(&self, app_state: &'a AppState, info: &UserInfo) -> Result<UserInfo> {
        let users_repo = app_state.users_repo();
        users_repo.add(info).await
    }

    async fn find<'a>(&self, app_state: &'a AppState, id: &str) -> Result<UserInfo> {
        let users_repo = app_state.users_repo();
        users_repo.find(id).await
    }
}
