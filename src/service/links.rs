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

    async fn post<'a>(&self, app_state: &'a state::Router, item: &LinkItem) -> Result<LinkItem> {
        let links_repo = app_state.get_links_repo();
        links_repo.post(item).await
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

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use crate::types::{
        links::LinkItem, repository::MockLinks as MockRepository,
        service::MockLinks as MockService, state::Router as RouterState,
    };

    use super::*;

    #[tokio::test]
    async fn test_get_links_empty() {
        let mock_links_service = MockService::new();
        let mut mock_links_repo = MockRepository::new();
        mock_links_repo
            .expect_list()
            .times(1)
            .returning(|| Ok(vec![]));

        let app_state = RouterState::new(Arc::new(mock_links_service), Arc::new(mock_links_repo));

        let links_service = Service {};
        let response = links_service.list(&app_state).await;

        assert!(response.is_ok());
        assert!(response.unwrap().is_empty());
    }

    #[tokio::test]
    async fn test_get_links_non_empty() {
        let mock_links_service = MockService::new();
        let mut mock_links_repo = MockRepository::new();

        let item: LinkItem = "http://link".into();
        let expected_items = vec![item.clone()];

        mock_links_repo
            .expect_list()
            .times(1)
            .returning(move || Ok(vec![item.clone()]));

        let app_state = RouterState::new(Arc::new(mock_links_service), Arc::new(mock_links_repo));

        let links_service = Service {};
        let response = links_service.list(&app_state).await;

        assert!(response.is_ok());

        let returned_items = response.unwrap();
        assert!(!returned_items.is_empty());
        assert!(returned_items
            .iter()
            .all(|item| expected_items.contains(item)));
    }

    #[tokio::test]
    async fn test_get_links_repo_error() {
        let mock_links_service = MockService::new();
        let mut mock_links_repo = MockRepository::new();
        mock_links_repo
            .expect_list()
            .times(1)
            .returning(|| Err("A service error occurred.".into()));

        let app_state = RouterState::new(Arc::new(mock_links_service), Arc::new(mock_links_repo));

        let links_service = Service {};
        let response = links_service.list(&app_state).await;

        assert!(response.is_err());
    }

    #[tokio::test]
    async fn test_post_links() {
        let mock_links_service = MockService::new();
        let mut mock_links_repo = MockRepository::new();
        let item: LinkItem = "http://link".into();
        let request_item = item.clone();
        let response_item = item.clone();

        mock_links_repo
            .expect_post()
            .times(1)
            .returning(move |_| Ok(item.clone()));

        let app_state = RouterState::new(Arc::new(mock_links_service), Arc::new(mock_links_repo));

        let links_service = Service {};
        let response = links_service.post(&app_state, &request_item).await;

        assert!(response.is_ok());
        assert_eq!(response.unwrap(), response_item);
    }

    #[tokio::test]
    async fn test_post_links_repo_error() {
        let mock_links_service = MockService::new();
        let mut mock_links_repo = MockRepository::new();
        let request_item: LinkItem = "http://link".into();

        mock_links_repo
            .expect_post()
            .times(1)
            .returning(|_| Err("A service error occurred.".into()));

        let app_state = RouterState::new(Arc::new(mock_links_service), Arc::new(mock_links_repo));

        let links_service = Service {};
        let response = links_service.post(&app_state, &request_item).await;

        assert!(response.is_err());
    }
}
