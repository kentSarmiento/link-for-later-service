use std::sync::Arc;

use axum::async_trait;
use serde::{Deserialize, Serialize};

pub type DynLinksRepo = Arc<dyn LinksRepo + Send + Sync>;
type RepoResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[async_trait]
pub trait LinksRepo {
    async fn list(&self) -> RepoResult<Vec<ListItem>>;
    async fn post(&self) -> RepoResult<ListItem>;
    async fn get(&self, id: &str) -> RepoResult<ListItem>;
    async fn put(&self, id: &str) -> RepoResult<ListItem>;
    async fn delete(&self, id: &str) -> RepoResult<()>;
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

pub struct SampleRepo {}

#[async_trait]
impl LinksRepo for SampleRepo {
    async fn list(&self) -> RepoResult<Vec<ListItem>> {
        Ok(vec![])
    }

    async fn post(&self) -> RepoResult<ListItem> {
        Ok(ListItem::default())
    }

    async fn get(&self, _id: &str) -> RepoResult<ListItem> {
        Ok(ListItem::default())
    }

    async fn put(&self, _id: &str) -> RepoResult<ListItem> {
        Ok(ListItem::default())
    }

    async fn delete(&self, _id: &str) -> RepoResult<()> {
        Ok(())
    }
}
