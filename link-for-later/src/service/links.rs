use axum::async_trait;
use chrono::Utc;

use crate::{
    dto::{LinkQuery, LinkQueryBuilder},
    entity::{LinkItem, LinkItemBuilder},
    repository, service,
    service::Links as LinksService,
    types::Result,
};

#[derive(Default)]
pub struct ServiceProvider {}

#[async_trait]
impl LinksService for ServiceProvider {
    async fn search(
        &self,
        links_repo: Box<repository::DynLinks>,
        link_query: &LinkQuery,
    ) -> Result<Vec<LinkItem>> {
        links_repo.find(link_query).await
    }

    async fn get(
        &self,
        links_repo: Box<repository::DynLinks>,
        link_query: &LinkQuery,
    ) -> Result<LinkItem> {
        links_repo.get(link_query).await
    }

    async fn create(
        &self,
        analysis_service: Box<service::DynAnalysis>,
        links_repo: Box<repository::DynLinks>,
        link_item: &LinkItem,
    ) -> Result<LinkItem> {
        let now = Utc::now();
        let created_link_item = LinkItemBuilder::from(link_item.clone())
            .created_at(&now)
            .updated_at(&now)
            .build();

        let created_link_item = links_repo.create(&created_link_item).await?;

        analysis_service.analyze(&created_link_item).await?;

        Ok(created_link_item)
    }

    async fn update(
        &self,
        analysis_service: Box<service::DynAnalysis>,
        links_repo: Box<repository::DynLinks>,
        id: &str,
        link_item: &LinkItem,
    ) -> Result<LinkItem> {
        let link_query = LinkQueryBuilder::new(id, link_item.owner()).build();
        let retrieved_link_item = links_repo.get(&link_query).await?;

        let now = Utc::now();
        let updated_link_item = LinkItemBuilder::from(link_item.clone())
            .created_at(retrieved_link_item.created_at())
            .updated_at(&now)
            .build();

        let updated_link_item = links_repo.update(id, &updated_link_item).await?;

        if updated_link_item.url() != retrieved_link_item.url() {
            analysis_service.analyze(&updated_link_item).await?;
        }

        Ok(updated_link_item)
    }

    async fn delete(
        &self,
        links_repo: Box<repository::DynLinks>,
        link_item: &LinkItem,
    ) -> Result<()> {
        let link_query = LinkQueryBuilder::new(link_item.id(), link_item.owner()).build();
        links_repo.get(&link_query).await?;
        links_repo.delete(link_item).await
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use mockall::Sequence;

    use crate::{
        repository::MockLinks as MockLinksRepo, service::MockAnalysis as MockAnalysisService,
        types::AppError,
    };

    use super::*;

    #[tokio::test]
    async fn test_search_links_empty() {
        let repo_query = LinkQueryBuilder::default().owner("user-id").build();
        let expected_query = LinkQueryBuilder::default().owner("user-id").build();

        let mut mock_links_repo = MockLinksRepo::new();
        mock_links_repo
            .expect_find()
            .withf(move |query| query == &expected_query)
            .times(1)
            .returning(|_| Ok(vec![]));

        let links_service = ServiceProvider {};
        let response = links_service
            .search(Box::new(Arc::new(mock_links_repo)), &repo_query)
            .await;

        assert!(response.is_ok());
        assert!(response.unwrap().is_empty());
    }

    #[tokio::test]
    async fn test_search_links_non_empty() {
        let repo_query = LinkQueryBuilder::default().owner("user-id").build();
        let expected_query = LinkQueryBuilder::default().owner("user-id").build();

        let item = LinkItemBuilder::new("http://link")
            .id("1")
            .owner("user-id")
            .build();
        let expected_items = vec![item.clone()];

        let mut mock_links_repo = MockLinksRepo::new();
        mock_links_repo
            .expect_find()
            .withf(move |query| query == &expected_query)
            .times(1)
            .returning(move |_| Ok(vec![item.clone()]));

        let links_service = ServiceProvider {};
        let response = links_service
            .search(Box::new(Arc::new(mock_links_repo)), &repo_query)
            .await;

        assert!(response.is_ok());

        let returned_items = response.unwrap();
        assert!(!returned_items.is_empty());
        assert!(returned_items
            .iter()
            .all(|item| expected_items.contains(item)));
    }

    #[tokio::test]
    async fn test_search_links_repo_error() {
        let repo_query = LinkQueryBuilder::default().owner("user-id").build();
        let expected_query = LinkQueryBuilder::default().owner("user-id").build();

        let mut mock_links_repo = MockLinksRepo::new();
        mock_links_repo
            .expect_find()
            .withf(move |query| query == &expected_query)
            .times(1)
            .returning(|_| Err(AppError::Test));

        let links_service = ServiceProvider {};
        let response = links_service
            .search(Box::new(Arc::new(mock_links_repo)), &repo_query)
            .await;

        assert_eq!(response, Err(AppError::Test));
    }

    #[tokio::test]
    async fn test_get_link() {
        let repo_query = LinkQueryBuilder::new("1", "user-id").build();
        let retrieved_item = LinkItemBuilder::new("http://link")
            .id("1")
            .owner("user-id")
            .build();
        let request_query = repo_query.clone();
        let response_item = retrieved_item.clone();

        let mut mock_links_repo = MockLinksRepo::new();
        mock_links_repo
            .expect_get()
            .withf(move |query| query == &repo_query)
            .times(1)
            .returning(move |_| Ok(retrieved_item.clone()));

        let links_service = ServiceProvider {};
        let response = links_service
            .get(Box::new(Arc::new(mock_links_repo)), &request_query)
            .await;

        assert!(response.is_ok());
        assert_eq!(response.unwrap(), response_item);
    }

    #[tokio::test]
    async fn test_get_link_not_found() {
        let repo_query = LinkQueryBuilder::new("1", "user-id").build();
        let request_query = repo_query.clone();

        let mut mock_links_repo = MockLinksRepo::new();
        mock_links_repo
            .expect_get()
            .withf(move |query| query == &repo_query)
            .times(1)
            .returning(|_| Err(AppError::LinkNotFound("1".into())));

        let links_service = ServiceProvider {};
        let response = links_service
            .get(Box::new(Arc::new(mock_links_repo)), &request_query)
            .await;

        assert_eq!(response, Err(AppError::LinkNotFound("1".into())));
    }

    #[tokio::test]
    async fn test_create_link() {
        let item_to_create = LinkItemBuilder::new("http://link").owner("user-id").build();
        let created_item = LinkItemBuilder::new("http://link")
            .id("1")
            .owner("user-id")
            .build();
        let item_to_analyze = created_item.clone();
        let request_item = item_to_create.clone();
        let response_item = created_item.clone();

        let mut seq = Sequence::new();

        let mut mock_links_repo = MockLinksRepo::new();
        mock_links_repo
            .expect_create()
            .withf(move |item| {
                item.url() == item_to_create.url() && item.owner() == item_to_create.owner()
            })
            .times(1)
            .in_sequence(&mut seq)
            .returning(move |_| Ok(created_item.clone()));

        let mut mock_analysis_service = MockAnalysisService::new();
        mock_analysis_service
            .expect_analyze()
            .withf(move |item| {
                item.id() == item_to_analyze.id()
                    && item.url() == item_to_analyze.url()
                    && item.owner() == item_to_analyze.owner()
            })
            .times(1)
            .in_sequence(&mut seq)
            .returning(|_| Ok(()));

        let links_service = ServiceProvider {};
        let response = links_service
            .create(
                Box::new(Arc::new(mock_analysis_service)),
                Box::new(Arc::new(mock_links_repo)),
                &request_item,
            )
            .await;

        assert!(response.is_ok());
        assert_eq!(response.unwrap(), response_item);
    }

    #[tokio::test]
    async fn test_create_link_repo_error() {
        let item_to_create = LinkItemBuilder::new("http://link").owner("user-id").build();
        let request_item = item_to_create.clone();

        let mut mock_links_repo = MockLinksRepo::new();
        mock_links_repo
            .expect_create()
            .withf(move |item| {
                item.url() == item_to_create.url() && item.owner() == item_to_create.owner()
            })
            .times(1)
            .returning(|_| Err(AppError::Test));

        let mut mock_analysis_service = MockAnalysisService::new();
        mock_analysis_service.expect_analyze().times(0);

        let links_service = ServiceProvider {};
        let response = links_service
            .create(
                Box::new(Arc::new(mock_analysis_service)),
                Box::new(Arc::new(mock_links_repo)),
                &request_item,
            )
            .await;

        assert_eq!(response, Err(AppError::Test));
    }

    #[tokio::test]
    async fn test_create_link_analyze_error() {
        let item_to_create = LinkItemBuilder::new("http://link").owner("user-id").build();
        let created_item = LinkItemBuilder::new("http://link")
            .id("1")
            .owner("user-id")
            .build();
        let item_to_analyze = created_item.clone();
        let request_item = item_to_create.clone();

        let mut seq = Sequence::new();

        let mut mock_links_repo = MockLinksRepo::new();
        mock_links_repo
            .expect_create()
            .withf(move |item| {
                item.url() == item_to_create.url() && item.owner() == item_to_create.owner()
            })
            .times(1)
            .in_sequence(&mut seq)
            .returning(move |_| Ok(created_item.clone()));

        let mut mock_analysis_service = MockAnalysisService::new();
        mock_analysis_service
            .expect_analyze()
            .withf(move |item| {
                item.id() == item_to_analyze.id()
                    && item.url() == item_to_analyze.url()
                    && item.owner() == item_to_analyze.owner()
            })
            .times(1)
            .in_sequence(&mut seq)
            .returning(|_| Err(AppError::Test));

        let links_service = ServiceProvider {};
        let response = links_service
            .create(
                Box::new(Arc::new(mock_analysis_service)),
                Box::new(Arc::new(mock_links_repo)),
                &request_item,
            )
            .await;

        assert_eq!(response, Err(AppError::Test));
    }

    #[tokio::test]
    async fn test_update_link() {
        let repo_query = LinkQueryBuilder::new("1", "user-id").build();
        let item_to_update = LinkItemBuilder::new("http://link")
            .owner("user-id")
            .description("sample link")
            .build();
        let retrieved_item = LinkItemBuilder::new("http://link")
            .id("1")
            .owner("user-id")
            .build();
        let updated_item = LinkItemBuilder::new("http://link")
            .id("1")
            .owner("user-id")
            .description("sample link")
            .build();
        let request_item = item_to_update.clone();
        let response_item = updated_item.clone();

        let mut seq = Sequence::new();

        let mut mock_links_repo = MockLinksRepo::new();
        mock_links_repo
            .expect_get()
            .withf(move |query| query == &repo_query)
            .times(1)
            .in_sequence(&mut seq)
            .returning(move |_| Ok(retrieved_item.clone()));
        mock_links_repo
            .expect_update()
            .withf(move |id, item| {
                id == "1"
                    && item.id() == item_to_update.id()
                    && item.url() == item_to_update.url()
                    && item.owner() == item_to_update.owner()
            })
            .times(1)
            .in_sequence(&mut seq)
            .returning(move |_, _| Ok(updated_item.clone()));

        let mut mock_analysis_service = MockAnalysisService::new();
        mock_analysis_service.expect_analyze().times(0);

        let links_service = ServiceProvider {};
        let response = links_service
            .update(
                Box::new(Arc::new(mock_analysis_service)),
                Box::new(Arc::new(mock_links_repo)),
                "1",
                &request_item,
            )
            .await;

        assert!(response.is_ok());
        assert_eq!(response.unwrap(), response_item);
    }

    #[tokio::test]
    async fn test_update_link_update_url() {
        let repo_query = LinkQueryBuilder::new("1", "user-id").build();
        let item_to_update = LinkItemBuilder::new("http://updated-link")
            .owner("user-id")
            .build();
        let retrieved_item = LinkItemBuilder::new("http://link")
            .id("1")
            .owner("user-id")
            .build();
        let updated_item = LinkItemBuilder::new("http://updated-link")
            .id("1")
            .owner("user-id")
            .build();
        let item_to_analyze = updated_item.clone();
        let request_item = item_to_update.clone();
        let response_item = updated_item.clone();

        let mut seq = Sequence::new();

        let mut mock_links_repo = MockLinksRepo::new();
        mock_links_repo
            .expect_get()
            .withf(move |query| query == &repo_query)
            .times(1)
            .in_sequence(&mut seq)
            .returning(move |_| Ok(retrieved_item.clone()));
        mock_links_repo
            .expect_update()
            .withf(move |id, item| {
                id == "1"
                    && item.id() == item_to_update.id()
                    && item.url() == item_to_update.url()
                    && item.owner() == item_to_update.owner()
            })
            .times(1)
            .in_sequence(&mut seq)
            .returning(move |_, _| Ok(updated_item.clone()));

        let mut mock_analysis_service = MockAnalysisService::new();
        mock_analysis_service
            .expect_analyze()
            .withf(move |item| {
                item.id() == item_to_analyze.id()
                    && item.url() == item_to_analyze.url()
                    && item.owner() == item_to_analyze.owner()
            })
            .times(1)
            .in_sequence(&mut seq)
            .returning(|_| Ok(()));

        let links_service = ServiceProvider {};
        let response = links_service
            .update(
                Box::new(Arc::new(mock_analysis_service)),
                Box::new(Arc::new(mock_links_repo)),
                "1",
                &request_item,
            )
            .await;

        assert!(response.is_ok());
        assert_eq!(response.unwrap(), response_item);
    }

    #[tokio::test]
    async fn test_update_link_not_found() {
        let repo_query = LinkQueryBuilder::new("1", "user-id").build();
        let item_to_update = LinkItemBuilder::new("http://link")
            .owner("user-id")
            .description("sample link")
            .build();
        let request_item = item_to_update.clone();

        let mut mock_links_repo = MockLinksRepo::new();
        mock_links_repo
            .expect_get()
            .withf(move |query| query == &repo_query)
            .times(1)
            .returning(|_| Err(AppError::LinkNotFound("1".into())));
        mock_links_repo.expect_update().times(0);

        let mut mock_analysis_service = MockAnalysisService::new();
        mock_analysis_service.expect_analyze().times(0);

        let links_service = ServiceProvider {};
        let response = links_service
            .update(
                Box::new(Arc::new(mock_analysis_service)),
                Box::new(Arc::new(mock_links_repo)),
                "1",
                &request_item,
            )
            .await;

        assert_eq!(response, Err(AppError::LinkNotFound("1".into())));
    }

    #[tokio::test]
    async fn test_update_link_repo_error() {
        let repo_query = LinkQueryBuilder::new("1", "user-id").build();
        let item_to_update = LinkItemBuilder::new("http://link")
            .owner("user-id")
            .description("sample link")
            .build();
        let retrieved_item = LinkItemBuilder::new("http://link")
            .id("1")
            .owner("user-id")
            .build();
        let request_item = item_to_update.clone();

        let mut seq = Sequence::new();

        let mut mock_links_repo = MockLinksRepo::new();
        mock_links_repo
            .expect_get()
            .withf(move |query| query == &repo_query)
            .times(1)
            .in_sequence(&mut seq)
            .returning(move |_| Ok(retrieved_item.clone()));
        mock_links_repo
            .expect_update()
            .withf(move |id, item| {
                id == "1"
                    && item.id() == item_to_update.id()
                    && item.url() == item_to_update.url()
                    && item.owner() == item_to_update.owner()
            })
            .times(1)
            .in_sequence(&mut seq)
            .returning(|_, _| Err(AppError::Test));

        let mut mock_analysis_service = MockAnalysisService::new();
        mock_analysis_service.expect_analyze().times(0);

        let links_service = ServiceProvider {};
        let response = links_service
            .update(
                Box::new(Arc::new(mock_analysis_service)),
                Box::new(Arc::new(mock_links_repo)),
                "1",
                &request_item,
            )
            .await;

        assert_eq!(response, Err(AppError::Test));
    }

    #[tokio::test]
    async fn test_update_link_analyze_error() {
        let repo_query = LinkQueryBuilder::new("1", "user-id").build();
        let item_to_update = LinkItemBuilder::new("http://updated-link")
            .owner("user-id")
            .build();
        let retrieved_item = LinkItemBuilder::new("http://link")
            .id("1")
            .owner("user-id")
            .build();
        let updated_item = LinkItemBuilder::new("http://updated-link")
            .id("1")
            .owner("user-id")
            .build();
        let item_to_analyze = updated_item.clone();
        let request_item = item_to_update.clone();

        let mut seq = Sequence::new();

        let mut mock_links_repo = MockLinksRepo::new();
        mock_links_repo
            .expect_get()
            .withf(move |query| query == &repo_query)
            .times(1)
            .in_sequence(&mut seq)
            .returning(move |_| Ok(retrieved_item.clone()));
        mock_links_repo
            .expect_update()
            .withf(move |id, item| {
                id == "1"
                    && item.id() == item_to_update.id()
                    && item.url() == item_to_update.url()
                    && item.owner() == item_to_update.owner()
            })
            .times(1)
            .in_sequence(&mut seq)
            .returning(move |_, _| Ok(updated_item.clone()));

        let mut mock_analysis_service = MockAnalysisService::new();
        mock_analysis_service
            .expect_analyze()
            .withf(move |item| {
                item.id() == item_to_analyze.id()
                    && item.url() == item_to_analyze.url()
                    && item.owner() == item_to_analyze.owner()
            })
            .times(1)
            .in_sequence(&mut seq)
            .returning(|_| Err(AppError::Test));

        let links_service = ServiceProvider {};
        let response = links_service
            .update(
                Box::new(Arc::new(mock_analysis_service)),
                Box::new(Arc::new(mock_links_repo)),
                "1",
                &request_item,
            )
            .await;

        assert_eq!(response, Err(AppError::Test));
    }

    #[tokio::test]
    async fn test_delete_link() {
        let repo_query = LinkQueryBuilder::new("1", "user-id").build();
        let item_to_delete = LinkItemBuilder::new("http://link")
            .id("1")
            .owner("user-id")
            .build();
        let retrieved_item = LinkItemBuilder::new("http://link")
            .id("1")
            .owner("user-id")
            .build();
        let request_item = item_to_delete.clone();

        let mut seq = Sequence::new();

        let mut mock_links_repo = MockLinksRepo::new();
        mock_links_repo
            .expect_get()
            .withf(move |query| query == &repo_query)
            .times(1)
            .in_sequence(&mut seq)
            .returning(move |_| Ok(retrieved_item.clone()));
        mock_links_repo
            .expect_delete()
            .withf(move |item| {
                item.id() == item_to_delete.id()
                    && item.url() == item_to_delete.url()
                    && item.owner() == item_to_delete.owner()
            })
            .times(1)
            .in_sequence(&mut seq)
            .returning(move |_| Ok(()));

        let links_service = ServiceProvider {};
        let response = links_service
            .delete(Box::new(Arc::new(mock_links_repo)), &request_item)
            .await;

        assert!(response.is_ok());
    }

    #[tokio::test]
    async fn test_delete_link_not_found() {
        let repo_query = LinkQueryBuilder::new("1", "user-id").build();
        let item_to_delete = LinkItemBuilder::new("http://link")
            .id("1")
            .owner("user-id")
            .build();
        let request_item = item_to_delete.clone();

        let mut mock_links_repo = MockLinksRepo::new();
        mock_links_repo
            .expect_get()
            .withf(move |query| query == &repo_query)
            .times(1)
            .returning(|_| Err(AppError::LinkNotFound("1".into())));
        mock_links_repo
            .expect_delete()
            .withf(move |item| {
                item.id() == item_to_delete.id()
                    && item.url() == item_to_delete.url()
                    && item.owner() == item_to_delete.owner()
            })
            .times(0);

        let links_service = ServiceProvider {};
        let response = links_service
            .delete(Box::new(Arc::new(mock_links_repo)), &request_item)
            .await;

        assert_eq!(response, Err(AppError::LinkNotFound("1".into())));
    }

    #[tokio::test]
    async fn test_delete_link_repo_error() {
        let repo_query = LinkQueryBuilder::new("1", "user-id").build();
        let item_to_delete = LinkItemBuilder::new("http://link")
            .id("1")
            .owner("user-id")
            .build();
        let retrieved_item = LinkItemBuilder::new("http://link")
            .id("1")
            .owner("user-id")
            .build();
        let request_item = item_to_delete.clone();

        let mut seq = Sequence::new();

        let mut mock_links_repo = MockLinksRepo::new();
        mock_links_repo
            .expect_get()
            .withf(move |query| query == &repo_query)
            .times(1)
            .in_sequence(&mut seq)
            .returning(move |_| Ok(retrieved_item.clone()));
        mock_links_repo
            .expect_delete()
            .withf(move |item| {
                item.id() == item_to_delete.id()
                    && item.url() == item_to_delete.url()
                    && item.owner() == item_to_delete.owner()
            })
            .times(1)
            .in_sequence(&mut seq)
            .returning(|_| Err(AppError::Test));

        let links_service = ServiceProvider {};
        let response = links_service
            .delete(Box::new(Arc::new(mock_links_repo)), &request_item)
            .await;

        assert_eq!(response, Err(AppError::Test));
    }
}
