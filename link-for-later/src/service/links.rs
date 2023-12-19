use axum::async_trait;

use crate::types::{links::LinkItem, service::Links, AppState, Result};

pub struct Service {}

#[async_trait]
impl Links for Service {
    async fn list<'a>(&self, app_state: &'a AppState) -> Result<Vec<LinkItem>> {
        let links_repo = app_state.get_links_repo();
        links_repo.list().await
    }

    async fn post<'a>(&self, app_state: &'a AppState, link_item: &LinkItem) -> Result<LinkItem> {
        let links_repo = app_state.get_links_repo();
        links_repo.post(link_item).await
    }

    async fn get<'a>(&self, app_state: &'a AppState, id: &str) -> Result<LinkItem> {
        let links_repo = app_state.get_links_repo();
        links_repo.get(id).await
    }

    async fn put<'a>(
        &self,
        app_state: &'a AppState,
        id: &str,
        link_item: &LinkItem,
    ) -> Result<LinkItem> {
        let links_repo = app_state.get_links_repo();

        let repo_item = links_repo.get(id).await?;
        let repo_item = repo_item.merge(link_item);

        links_repo.post(&repo_item).await
    }

    async fn delete<'a>(&self, app_state: &'a AppState, id: &str) -> Result<()> {
        let links_repo = app_state.get_links_repo();

        links_repo.get(id).await?;
        links_repo.delete(id).await
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use crate::types::{
        links::LinkItem, repository::MockLinks as MockRepository,
        service::MockLinks as MockService, AppError, AppState,
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

        let app_state = AppState::new(Arc::new(mock_links_service), Arc::new(mock_links_repo));

        let links_service = Service {};
        let response = links_service.list(&app_state).await;

        assert!(response.is_ok());
        assert!(response.unwrap().is_empty());
    }

    #[tokio::test]
    async fn test_get_links_non_empty() {
        let mock_links_service = MockService::new();
        let mut mock_links_repo = MockRepository::new();

        let item = LinkItem::new("1", "http://link");
        let expected_items = vec![item.clone()];

        mock_links_repo
            .expect_list()
            .times(1)
            .returning(move || Ok(vec![item.clone()]));

        let app_state = AppState::new(Arc::new(mock_links_service), Arc::new(mock_links_repo));

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
            .returning(|| Err(AppError::TestError));

        let app_state = AppState::new(Arc::new(mock_links_service), Arc::new(mock_links_repo));

        let links_service = Service {};
        let response = links_service.list(&app_state).await;

        assert!(response.is_err());
    }

    #[tokio::test]
    async fn test_post_links() {
        let mock_links_service = MockService::new();
        let mut mock_links_repo = MockRepository::new();
        let item = LinkItem::new("1", "http://link");
        let request_item = item.clone();
        let response_item = item.clone();

        mock_links_repo
            .expect_post()
            .times(1)
            .returning(move |_| Ok(item.clone()));

        let app_state = AppState::new(Arc::new(mock_links_service), Arc::new(mock_links_repo));

        let links_service = Service {};
        let response = links_service.post(&app_state, &request_item).await;

        assert!(response.is_ok());
        assert_eq!(response.unwrap(), response_item);
    }

    #[tokio::test]
    async fn test_post_links_repo_error() {
        let mock_links_service = MockService::new();
        let mut mock_links_repo = MockRepository::new();
        let request_item = LinkItem::new("1", "http://link");

        mock_links_repo
            .expect_post()
            .times(1)
            .returning(|_| Err(AppError::TestError));

        let app_state = AppState::new(Arc::new(mock_links_service), Arc::new(mock_links_repo));

        let links_service = Service {};
        let response = links_service.post(&app_state, &request_item).await;

        assert!(response.is_err());
    }

    #[tokio::test]
    async fn test_get_link_not_found() {
        let mock_links_service = MockService::new();
        let mut mock_links_repo = MockRepository::new();
        mock_links_repo
            .expect_get()
            .times(1)
            .returning(|_| Err(AppError::ItemNotFound));

        let app_state = AppState::new(Arc::new(mock_links_service), Arc::new(mock_links_repo));

        let links_service = Service {};
        let response = links_service.get(&app_state, "1111").await;

        assert!(response.is_err());
    }

    #[tokio::test]
    async fn test_get_link_found() {
        let mock_links_service = MockService::new();
        let mut mock_links_repo = MockRepository::new();
        let item = LinkItem::new("1", "http://link");
        let response_item = item.clone();

        mock_links_repo
            .expect_get()
            .times(1)
            .returning(move |_| Ok(item.clone()));

        let app_state = AppState::new(Arc::new(mock_links_service), Arc::new(mock_links_repo));

        let links_service = Service {};
        let response = links_service.get(&app_state, "1111").await;

        assert!(response.is_ok());
        assert_eq!(response.unwrap(), response_item);
    }

    #[tokio::test]
    async fn test_put_links() {
        let mock_links_service = MockService::new();
        let mut mock_links_repo = MockRepository::new();

        let get_item = LinkItem::new("1", "http://link");
        let post_item = get_item.clone();
        let request_item = get_item.clone();
        let response_item = get_item.clone();

        mock_links_repo
            .expect_get()
            .times(1)
            .returning(move |_| Ok(get_item.clone()));
        mock_links_repo
            .expect_post()
            .times(1)
            .returning(move |_| Ok(post_item.clone()));

        let app_state = AppState::new(Arc::new(mock_links_service), Arc::new(mock_links_repo));

        let links_service = Service {};
        let response = links_service.put(&app_state, "1111", &request_item).await;

        assert!(response.is_ok());
        assert_eq!(response.unwrap(), response_item);
    }

    #[tokio::test]
    async fn test_put_links_not_found() {
        let mock_links_service = MockService::new();
        let mut mock_links_repo = MockRepository::new();
        let request_item = LinkItem::new("1", "http://link");

        mock_links_repo
            .expect_get()
            .times(1)
            .returning(|_| Err(AppError::ItemNotFound));
        mock_links_repo.expect_post().times(0);

        let app_state = AppState::new(Arc::new(mock_links_service), Arc::new(mock_links_repo));

        let links_service = Service {};
        let response = links_service.put(&app_state, "1111", &request_item).await;

        assert!(response.is_err());
    }

    #[tokio::test]
    async fn test_put_links_repo_error() {
        let mock_links_service = MockService::new();
        let mut mock_links_repo = MockRepository::new();

        let item = LinkItem::new("1", "http://link");
        let request_item = item.clone();

        mock_links_repo
            .expect_get()
            .times(1)
            .returning(move |_| Ok(item.clone()));
        mock_links_repo
            .expect_post()
            .times(1)
            .returning(|_| Err(AppError::TestError));

        let app_state = AppState::new(Arc::new(mock_links_service), Arc::new(mock_links_repo));

        let links_service = Service {};
        let response = links_service.put(&app_state, "1111", &request_item).await;

        assert!(response.is_err());
    }

    #[tokio::test]
    async fn test_delete_links_not_found() {
        let mock_links_service = MockService::new();
        let mut mock_links_repo = MockRepository::new();

        mock_links_repo
            .expect_get()
            .times(1)
            .returning(|_| Err(AppError::ItemNotFound));
        mock_links_repo.expect_delete().times(0);

        let app_state = AppState::new(Arc::new(mock_links_service), Arc::new(mock_links_repo));

        let links_service = Service {};
        let response = links_service.delete(&app_state, "1111").await;

        assert!(response.is_err());
    }

    #[tokio::test]
    async fn test_delete_links_found() {
        let mock_links_service = MockService::new();
        let mut mock_links_repo = MockRepository::new();
        let item = LinkItem::new("1", "http://link");

        mock_links_repo
            .expect_get()
            .times(1)
            .returning(move |_| Ok(item.clone()));
        mock_links_repo
            .expect_delete()
            .times(1)
            .returning(|_| Ok(()));

        let app_state = AppState::new(Arc::new(mock_links_service), Arc::new(mock_links_repo));

        let links_service = Service {};
        let response = links_service.delete(&app_state, "1111").await;

        assert!(response.is_ok());
    }
}