use std::sync::Arc;

use axum::async_trait;
#[cfg(test)]
use mockall::{automock, predicate::*};

use super::{links::LinkItem, Result, RouterState};

pub type DynLinks = Arc<dyn Links + Send + Sync>;

#[cfg_attr(test, automock)]
#[async_trait]
pub trait Links {
    async fn list<'a>(&self, app_state: &'a RouterState) -> Result<Vec<LinkItem>>;

    async fn post<'a>(&self, app_state: &'a RouterState, link_item: &LinkItem) -> Result<LinkItem>;

    async fn get<'a>(&self, app_state: &'a RouterState, id: &str) -> Result<LinkItem>;

    async fn put<'a>(
        &self,
        app_state: &'a RouterState,
        id: &str,
        link_item: &LinkItem,
    ) -> Result<LinkItem>;

    async fn delete<'a>(&self, app_state: &'a RouterState, id: &str) -> Result<()>;
}
