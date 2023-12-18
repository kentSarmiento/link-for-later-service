use std::sync::Arc;

use axum::async_trait;
#[cfg(test)]
use mockall::{automock, predicate::*};

use super::links::LinkItem;

pub type DynLinks = Arc<dyn Links + Send + Sync>;
pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[cfg_attr(test, automock)]
#[async_trait]
pub trait Links {
    async fn list(&self) -> Result<Vec<LinkItem>>;

    async fn post(&self) -> Result<LinkItem>;

    async fn get(&self, id: &str) -> Result<LinkItem>;

    async fn put(&self, id: &str) -> Result<LinkItem>;

    async fn delete(&self, id: &str) -> Result<()>;
}
