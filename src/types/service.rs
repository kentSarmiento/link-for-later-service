use std::sync::Arc;

use axum::async_trait;
#[cfg(test)]
use mockall::{automock, predicate::*};

use super::{links::LinkItem, state};

pub type DynLinks = Arc<dyn Links + Send + Sync>;
pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[cfg_attr(test, automock)]
#[async_trait]
pub trait Links {
    async fn list<'a>(&self, app_state: &'a state::Router) -> Result<Vec<LinkItem>>;

    async fn post<'a>(&self, app_state: &'a state::Router) -> Result<LinkItem>;

    async fn get<'a>(&self, id: &str, app_state: &'a state::Router) -> Result<LinkItem>;

    async fn put<'a>(&self, id: &str, app_state: &'a state::Router) -> Result<LinkItem>;

    async fn delete<'a>(&self, id: &str, app_state: &'a state::Router) -> Result<()>;
}
