use axum::async_trait;
use chrono::Utc;

use crate::{
    repository,
    service::Links as LinksService,
    types::{entity::LinkItem, Result},
};

pub struct ServiceProvider {}

#[async_trait]
impl LinksService for ServiceProvider {
    async fn list(&self, links_repo: Box<repository::DynLinks>) -> Result<Vec<LinkItem>> {
        links_repo.list().await
    }

    async fn post(
        &self,
        links_repo: Box<repository::DynLinks>,
        link_item: &LinkItem,
    ) -> Result<LinkItem> {
        let now = Utc::now().to_rfc3339();
        let created_link_item = link_item.created_at(&now).updated_at(&now);

        links_repo.post(&created_link_item).await
    }

    async fn get(&self, links_repo: Box<repository::DynLinks>, id: &str) -> Result<LinkItem> {
        links_repo.get(id).await
    }

    async fn put(
        &self,
        links_repo: Box<repository::DynLinks>,
        id: &str,
        link_item: &LinkItem,
    ) -> Result<LinkItem> {
        let now = Utc::now().to_rfc3339();
        let updated_link_item = link_item.updated_at(&now);

        links_repo.put(id, &updated_link_item).await
    }

    async fn delete(&self, links_repo: Box<repository::DynLinks>, id: &str) -> Result<()> {
        links_repo.get(id).await?;
        links_repo.delete(id).await
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use crate::{
        repository::MockLinks as MockLinksRepo,
        types::{entity::LinkItem, AppError},
    };

    use super::*;

    #[tokio::test]
    async fn test_get_links_empty() {
        let mut mock_links_repo = MockLinksRepo::new();
        mock_links_repo
            .expect_list()
            .times(1)
            .returning(|| Ok(vec![]));

        let links_service = ServiceProvider {};
        let response = links_service
            .list(Box::new(Arc::new(mock_links_repo)))
            .await;

        assert!(response.is_ok());
        assert!(response.unwrap().is_empty());
    }

    #[tokio::test]
    async fn test_get_links_non_empty() {
        let item = LinkItem::new("1", "http://link");
        let expected_items = vec![item.clone()];

        let mut mock_links_repo = MockLinksRepo::new();
        mock_links_repo
            .expect_list()
            .times(1)
            .returning(move || Ok(vec![item.clone()]));

        let links_service = ServiceProvider {};
        let response = links_service
            .list(Box::new(Arc::new(mock_links_repo)))
            .await;

        assert!(response.is_ok());

        let returned_items = response.unwrap();
        assert!(!returned_items.is_empty());
        assert!(returned_items
            .iter()
            .all(|item| expected_items.contains(item)));
    }

    #[tokio::test]
    async fn test_links_repo_error() {
        let mut mock_links_repo = MockLinksRepo::new();
        mock_links_repo
            .expect_list()
            .times(1)
            .returning(|| Err(AppError::TestError));

        let links_service = ServiceProvider {};
        let response = links_service
            .list(Box::new(Arc::new(mock_links_repo)))
            .await;

        assert!(response.is_err());
    }

    #[tokio::test]
    async fn test_post_links() {
        let item = LinkItem::new("1", "http://link");
        let request_item = item.clone();
        let response_item = item.clone();

        let mut mock_links_repo = MockLinksRepo::new();
        mock_links_repo
            .expect_post()
            .times(1)
            .returning(move |_| Ok(item.clone()));

        let links_service = ServiceProvider {};
        let response = links_service
            .post(Box::new(Arc::new(mock_links_repo)), &request_item)
            .await;

        assert!(response.is_ok());
        assert_eq!(response.unwrap(), response_item);
    }

    #[tokio::test]
    async fn test_post_links_repo_error() {
        let request_item = LinkItem::new("1", "http://link");

        let mut mock_links_repo = MockLinksRepo::new();
        mock_links_repo
            .expect_post()
            .times(1)
            .returning(|_| Err(AppError::TestError));

        let links_service = ServiceProvider {};
        let response = links_service
            .post(Box::new(Arc::new(mock_links_repo)), &request_item)
            .await;

        assert!(response.is_err());
    }

    #[tokio::test]
    async fn test_get_link_not_found() {
        let mut mock_links_repo = MockLinksRepo::new();
        mock_links_repo
            .expect_get()
            .times(1)
            .returning(|_| Err(AppError::ItemNotFound));

        let links_service = ServiceProvider {};
        let response = links_service
            .get(Box::new(Arc::new(mock_links_repo)), "1111")
            .await;

        assert!(response.is_err());
    }

    #[tokio::test]
    async fn test_get_link_found() {
        let item = LinkItem::new("1", "http://link");
        let response_item = item.clone();

        let mut mock_links_repo = MockLinksRepo::new();
        mock_links_repo
            .expect_get()
            .times(1)
            .returning(move |_| Ok(item.clone()));

        let links_service = ServiceProvider {};
        let response = links_service
            .get(Box::new(Arc::new(mock_links_repo)), "1111")
            .await;

        assert!(response.is_ok());
        assert_eq!(response.unwrap(), response_item);
    }

    #[tokio::test]
    async fn test_put_links() {
        let put_item = LinkItem::new("1", "http://link");
        let request_item = put_item.clone();
        let response_item = put_item.clone();

        let mut mock_links_repo = MockLinksRepo::new();
        mock_links_repo
            .expect_put()
            .times(1)
            .returning(move |_, _| Ok(put_item.clone()));

        let links_service = ServiceProvider {};
        let response = links_service
            .put(Box::new(Arc::new(mock_links_repo)), "1111", &request_item)
            .await;

        assert!(response.is_ok());
        assert_eq!(response.unwrap(), response_item);
    }

    #[tokio::test]
    async fn test_put_links_repo_error() {
        let request_item = LinkItem::new("1", "http://link");

        let mut mock_links_repo = MockLinksRepo::new();
        mock_links_repo
            .expect_put()
            .times(1)
            .returning(|_, _| Err(AppError::TestError));

        let links_service = ServiceProvider {};
        let response = links_service
            .put(Box::new(Arc::new(mock_links_repo)), "1111", &request_item)
            .await;

        assert!(response.is_err());
    }

    #[tokio::test]
    async fn test_delete_links_not_found() {
        let mut mock_links_repo = MockLinksRepo::new();
        mock_links_repo
            .expect_get()
            .times(1)
            .returning(|_| Err(AppError::ItemNotFound));
        mock_links_repo.expect_delete().times(0);

        let links_service = ServiceProvider {};
        let response = links_service
            .delete(Box::new(Arc::new(mock_links_repo)), "1111")
            .await;

        assert!(response.is_err());
    }

    #[tokio::test]
    async fn test_delete_links_found() {
        let item = LinkItem::new("1", "http://link");

        let mut mock_links_repo = MockLinksRepo::new();
        mock_links_repo
            .expect_get()
            .times(1)
            .returning(move |_| Ok(item.clone()));
        mock_links_repo
            .expect_delete()
            .times(1)
            .returning(|_| Ok(()));

        let links_service = ServiceProvider {};
        let response = links_service
            .delete(Box::new(Arc::new(mock_links_repo)), "1111")
            .await;

        assert!(response.is_ok());
    }
}
