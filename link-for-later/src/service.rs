use std::sync::Arc;

use axum::async_trait;
#[cfg(test)]
use mockall::{automock, predicate::*};

use crate::{
    repository, service,
    types::{LinkItem, LinkQuery, Result, Token, UserInfo},
};

pub type DynLinks = Arc<dyn Links + Send + Sync>;
pub type DynUsers = Arc<dyn Users + Send + Sync>;
pub type DynAnalysis = Arc<dyn Analysis + Send + Sync>;

#[cfg_attr(test, automock)]
#[async_trait]
pub trait Links {
    async fn search(
        &self,
        links_repo: Box<repository::DynLinks>,
        query: &LinkQuery,
    ) -> Result<Vec<LinkItem>>;

    async fn get(
        &self,
        links_repo: Box<repository::DynLinks>,
        query: &LinkQuery,
    ) -> Result<LinkItem>;

    async fn create(
        &self,
        analysis_service: Box<service::DynAnalysis>,
        links_repo: Box<repository::DynLinks>,
        item: &LinkItem,
    ) -> Result<LinkItem>;

    async fn update(
        &self,
        analysis_service: Box<service::DynAnalysis>,
        links_repo: Box<repository::DynLinks>,
        query: &LinkQuery,
        item: &LinkItem,
    ) -> Result<LinkItem>;

    async fn delete(&self, links_repo: Box<repository::DynLinks>, query: &LinkQuery) -> Result<()>;
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
        user_info: &UserInfo,
    ) -> Result<Token>;
}

#[cfg_attr(test, automock)]
#[async_trait]
pub trait Analysis {
    async fn analyze(&self, link_item: &LinkItem) -> Result<()>;
}

pub mod analysis;
pub mod links;
pub mod users;
