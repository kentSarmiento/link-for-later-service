use std::sync::Arc;

use axum::async_trait;
#[cfg(test)]
use mockall::{automock, predicate::*};

use crate::{
    repository,
    types::{
        auth::Token,
        entity::{LinkItem, UserInfo},
        Result,
    },
};

pub type DynLinks = Arc<dyn Links + Send + Sync>;
pub type DynUsers = Arc<dyn Users + Send + Sync>;

#[cfg_attr(test, automock)]
#[async_trait]
pub trait Links {
    async fn list(&self, links_repo: Box<repository::DynLinks>) -> Result<Vec<LinkItem>>;

    async fn post(
        &self,
        links_repo: Box<repository::DynLinks>,
        link_item: &LinkItem,
    ) -> Result<LinkItem>;

    async fn get(&self, links_repo: Box<repository::DynLinks>, id: &str) -> Result<LinkItem>;

    async fn put(
        &self,
        links_repo: Box<repository::DynLinks>,
        id: &str,
        link_item: &LinkItem,
    ) -> Result<LinkItem>;

    async fn delete(&self, links_repo: Box<repository::DynLinks>, id: &str) -> Result<()>;
}

#[cfg_attr(test, automock)]
#[async_trait]
pub trait Users {
    async fn register(
        &self,
        users_repo: Box<repository::DynUsers>,
        user_info: &UserInfo,
    ) -> Result<UserInfo>;

    async fn login(
        &self,
        users_repo: Box<repository::DynUsers>,
        secret_key: &str,
        user_info: &UserInfo,
    ) -> Result<Token>;
}

pub mod links;
pub mod users;
