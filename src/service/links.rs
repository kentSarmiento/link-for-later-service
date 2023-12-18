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
}
