use axum::async_trait;

use crate::types::{
    links::LinkItem,
    service::{Links, Result},
    state,
};

pub struct Service {}

#[async_trait]
impl Links for Service {
    async fn list<'a>(&self, app_state: &'a state::Router) -> Result<Vec<LinkItem>> {
        let links_repo = app_state.get_links_repo();
        links_repo.list().await
    }

    async fn post<'a>(&self, app_state: &'a state::Router) -> Result<LinkItem> {
        let links_repo = app_state.get_links_repo();
        links_repo.post().await
    }

    async fn get<'a>(&self, id: &str, app_state: &'a state::Router) -> Result<LinkItem> {
        let links_repo = app_state.get_links_repo();
        links_repo.get(id).await
    }

    async fn put<'a>(&self, id: &str, app_state: &'a state::Router) -> Result<LinkItem> {
        let links_repo = app_state.get_links_repo();
        links_repo.put(id).await
    }

    async fn delete<'a>(&self, id: &str, app_state: &'a state::Router) -> Result<()> {
        let links_repo = app_state.get_links_repo();
        links_repo.delete(id).await
    }
}
