use std::sync::Arc;

use axum::async_trait;
#[cfg(test)]
use mockall::{automock, predicate::*};

use crate::{
    state::AppState,
    types::{
        entity::{LinkItem, UserInfo},
        Result,
    },
};

pub type DynLinks = Arc<dyn Links + Send + Sync>;
pub type DynUsers = Arc<dyn Users + Send + Sync>;

#[cfg_attr(test, automock)]
#[async_trait]
pub trait Links {
    async fn list<'a>(&self, app_state: &'a AppState) -> Result<Vec<LinkItem>>;
    async fn post<'a>(&self, app_state: &'a AppState, link_item: &LinkItem) -> Result<LinkItem>;
    async fn get<'a>(&self, app_state: &'a AppState, id: &str) -> Result<LinkItem>;
    async fn put<'a>(
        &self,
        app_state: &'a AppState,
        id: &str,
        link_item: &LinkItem,
    ) -> Result<LinkItem>;
    async fn delete<'a>(&self, app_state: &'a AppState, id: &str) -> Result<()>;
}

#[cfg_attr(test, automock)]
#[async_trait]
pub trait Users {
    async fn add<'a>(&self, app_state: &'a AppState, info: &UserInfo) -> Result<UserInfo>;
    async fn find<'a>(&self, app_state: &'a AppState, id: &str) -> Result<UserInfo>;
}

pub mod links;
pub mod users;
