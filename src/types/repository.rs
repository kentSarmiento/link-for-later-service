use std::sync::Arc;

use axum::async_trait;
#[cfg(test)]
use mockall::{automock, predicate::*};

use super::{links::LinkItem, Result};

pub type DynLinks = Arc<dyn Links + Send + Sync>;

#[cfg_attr(test, automock)]
#[async_trait]
pub trait Links {
    async fn list(&self) -> Result<Vec<LinkItem>>;

    async fn post(&self, item: &LinkItem) -> Result<LinkItem>;

    async fn get(&self, id: &str) -> Result<LinkItem>;

    async fn put(&self, id: &str) -> Result<LinkItem>;

    async fn delete(&self, id: &str) -> Result<()>;
}
