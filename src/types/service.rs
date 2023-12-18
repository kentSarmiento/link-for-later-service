use std::sync::Arc;

use axum::async_trait;

use super::{links::LinkItem, state};

pub type DynLinks = Arc<dyn Links + Send + Sync>;
pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[async_trait]
pub trait Links {
    async fn list<'a>(&self, _app_state: &'a state::Router) -> Result<Vec<LinkItem>> {
        Err("Not implemented".into())
    }

    async fn post<'a>(&self, _app_state: &'a state::Router) -> Result<LinkItem> {
        Err("Not implemented".into())
    }

    async fn get<'a>(&self, _id: &str, _app_state: &'a state::Router) -> Result<LinkItem> {
        Err("Not implemented".into())
    }

    async fn put<'a>(&self, _id: &str, _app_state: &'a state::Router) -> Result<LinkItem> {
        Err("Not implemented".into())
    }

    async fn delete<'a>(&self, _id: &str, _app_state: &'a state::Router) -> Result<()> {
        Err("Not implemented".into())
    }
}
