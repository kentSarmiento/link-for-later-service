use std::sync::Arc;

use axum::async_trait;
use serde::{Deserialize, Serialize};

pub mod sample;

pub type DynLinksRepo = Arc<dyn LinksRepo + Send + Sync>;
type RepoResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[async_trait]
pub trait LinksRepo {
    async fn list(&self) -> RepoResult<Vec<ListItem>> {
        Err("Not implemented".into())
    }

    async fn post(&self) -> RepoResult<ListItem> {
        Err("Not implemented".into())
    }

    async fn get(&self, _id: &str) -> RepoResult<ListItem> {
        Err("Not implemented".into())
    }

    async fn put(&self, _id: &str) -> RepoResult<ListItem> {
        Err("Not implemented".into())
    }

    async fn delete(&self, _id: &str) -> RepoResult<()> {
        Err("Not implemented".into())
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ListItem {
    id: String,
    owner: String,
    url: String,
    title: String,
    description: String,
    created_at: String,
    updated_at: String,
}
