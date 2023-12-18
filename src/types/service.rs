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

    async fn post(&self) -> Result<LinkItem> {
        Err("Not implemented".into())
    }

    async fn get(&self, _id: &str) -> Result<LinkItem> {
        Err("Not implemented".into())
    }

    async fn put(&self, _id: &str) -> Result<LinkItem> {
        Err("Not implemented".into())
    }

    async fn delete(&self, _id: &str) -> Result<()> {
        Err("Not implemented".into())
    }
}
