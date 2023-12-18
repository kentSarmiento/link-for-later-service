use axum::async_trait;

use crate::types::{
    links::LinkItem,
    repository::{Links, Result},
};

pub struct Repository {}

#[async_trait]
impl Links for Repository {
    async fn list(&self) -> Result<Vec<LinkItem>> {
        Ok(vec![])
    }

    async fn post(&self, item: &LinkItem) -> Result<LinkItem> {
        Ok(item.clone())
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
