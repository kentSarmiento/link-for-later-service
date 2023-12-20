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

#[allow(clippy::used_underscore_binding)]
#[cfg_attr(test, automock)]
#[async_trait]
pub trait Links {
    async fn list(&self) -> Result<Vec<LinkItem>>;
    async fn post(&self, _item: &LinkItem) -> Result<LinkItem>;
    async fn get(&self, _id: &str) -> Result<LinkItem>;
    async fn put(&self, _id: &str, _item: &LinkItem) -> Result<LinkItem>;
    async fn delete(&self, _id: &str) -> Result<()>;
}

#[allow(clippy::used_underscore_binding)]
#[cfg_attr(test, automock)]
#[async_trait]
pub trait Users {
    async fn add(&self, _info: &UserInfo) -> Result<UserInfo>;
    async fn find(&self, _id: &str) -> Result<UserInfo>;
}

pub mod bare;
pub use bare::Bare as BareDb;

pub mod mongodb;
