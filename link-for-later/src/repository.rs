use std::sync::Arc;

use axum::async_trait;
#[cfg(test)]
use mockall::{automock, predicate::*};

use crate::types::{
    entity::{LinkItem, UserInfo},
    Result,
};

pub type DynLinks = Arc<dyn Links + Send + Sync>;
pub type DynUsers = Arc<dyn Users + Send + Sync>;

#[cfg_attr(test, automock)]
#[async_trait]
pub trait Links {
    async fn list(&self) -> Result<Vec<LinkItem>>;
    async fn post(&self, item: &LinkItem) -> Result<LinkItem>;
    async fn get(&self, id: &str) -> Result<LinkItem>;
    async fn put(&self, id: &str, item: &LinkItem) -> Result<LinkItem>;
    async fn delete(&self, id: &str) -> Result<()>;
}

#[cfg_attr(test, automock)]
#[async_trait]
pub trait Users {
    async fn add(&self, info: &UserInfo) -> Result<UserInfo>;
    async fn find(&self, id: &str) -> Result<UserInfo>;
}

pub mod bare;
pub use bare::Bare as BareDb;

pub mod mongodb;
