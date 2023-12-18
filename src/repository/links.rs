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
}
