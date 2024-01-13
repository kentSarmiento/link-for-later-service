use std::sync::Arc;

use axum::async_trait;
#[cfg(test)]
use mockall::{automock, predicate::*};

use crate::types::{LinkItem, LinkQuery, Result, UserInfo, UserQuery};

pub type DynLinks = Arc<dyn Links + Send + Sync>;
pub type DynUsers = Arc<dyn Users + Send + Sync>;

#[cfg_attr(test, automock)]
#[async_trait]
pub trait Links {
    async fn find(&self, query: &LinkQuery) -> Result<Vec<LinkItem>>;
    async fn get(&self, query: &LinkQuery) -> Result<LinkItem>;
    async fn create(&self, item: &LinkItem) -> Result<LinkItem>;
    async fn update(&self, query: &LinkQuery, item: &LinkItem) -> Result<LinkItem>;
    async fn delete(&self, query: &LinkQuery) -> Result<()>;
}

#[cfg_attr(test, automock)]
#[async_trait]
pub trait Users {
    async fn get(&self, query: &UserQuery) -> Result<UserInfo>;
    async fn create(&self, info: &UserInfo) -> Result<UserInfo>;
}

pub mod inmemory;
pub mod mongodb;
