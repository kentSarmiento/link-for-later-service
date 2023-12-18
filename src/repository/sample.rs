use axum::async_trait;

use super::{LinksRepo, ListItem, RepoResult};

pub struct Repo {}

#[async_trait]
impl LinksRepo for Repo {
    async fn list(&self) -> RepoResult<Vec<ListItem>> {
        Ok(vec![])
    }
}
