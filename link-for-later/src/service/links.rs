use axum::async_trait;
use chrono::Utc;

use crate::{
    repository, service,
    service::Links as LinksService,
    types::{AppError, LinkItem, LinkItemBuilder, LinkQuery, LinkQueryBuilder, Result},
};

#[derive(Default)]
pub struct ServiceProvider {}

#[async_trait]
impl LinksService for ServiceProvider {
    async fn search(
        &self,
        links_repo: Box<repository::DynLinks>,
        query: &LinkQuery,
    ) -> Result<Vec<LinkItem>> {
        let find_query = if query.is_from_admin() {
            LinkQueryBuilder::default().user("").build()
        } else {
            LinkQueryBuilder::default().user(query.user()).build()
        };
        links_repo.find(&find_query).await
    }

    async fn get(
        &self,
        links_repo: Box<repository::DynLinks>,
        query: &LinkQuery,
    ) -> Result<LinkItem> {
        let get_query = LinkQueryBuilder::default().id(query.id()).build();
        let retrieved_item = links_repo.get(&get_query).await?;

        if query.user() == retrieved_item.owner() || query.is_from_admin() {
            Ok(retrieved_item)
        } else {
            Err(AppError::Authorization(String::from(
                "User is not authorized to access resource",
            )))
        }
    }

    async fn create(
        &self,
        analysis_service: Box<service::DynAnalysis>,
        links_repo: Box<repository::DynLinks>,
        item: &LinkItem,
    ) -> Result<LinkItem> {
        let now = Utc::now();
        let created_item = LinkItemBuilder::from(item.clone())
            .created_at(&now)
            .updated_at(&now)
            .build();

        let created_item = links_repo.create(&created_item).await?;

        analysis_service.analyze(&created_item).await?;

        Ok(created_item)
    }

    async fn update(
        &self,
        analysis_service: Box<service::DynAnalysis>,
        links_repo: Box<repository::DynLinks>,
        query: &LinkQuery,
        item: &LinkItem,
    ) -> Result<LinkItem> {
        let retrieved_item = self.get(links_repo.clone(), query).await?;

        let now = Utc::now();
        let updated_item = LinkItemBuilder::from(item.clone())
            .owner(retrieved_item.owner())
            .created_at(retrieved_item.created_at())
            .updated_at(&now)
            .build();

        let update_query = LinkQueryBuilder::default().id(query.id()).build();
        let updated_item = links_repo.update(&update_query, &updated_item).await?;

        if updated_item.url() != retrieved_item.url() {
            analysis_service.analyze(&updated_item).await?;
        }

        Ok(updated_item)
    }

    async fn delete(&self, links_repo: Box<repository::DynLinks>, query: &LinkQuery) -> Result<()> {
        self.get(links_repo.clone(), query).await?;

        let delete_query = LinkQueryBuilder::default().id(query.id()).build();
        links_repo.delete(&delete_query).await
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use mockall::Sequence;
    use rstest::rstest;

    use crate::{
        repository::MockLinks as MockLinksRepo, service::MockAnalysis as MockAnalysisService,
        types::AppError,
    };

    use super::*;

    #[rstest]
    #[case(true, "admin")]
    #[case(false, "user")]
    #[tokio::test]
    async fn test_search_links_empty(#[case] is_admin: bool, #[case] user: &str) {
        let request_query = LinkQueryBuilder::default()
            .user(user)
            .is_from_admin(is_admin)
            .build();
        let find_query = if is_admin {
            LinkQueryBuilder::default().user("").build()
        } else {
            LinkQueryBuilder::default().user(user).build()
        };

        let mut mock_links_repo = MockLinksRepo::new();
        mock_links_repo
            .expect_find()
            .withf(move |query| query == &find_query)
            .times(1)
            .returning(|_| Ok(vec![]));

        let links_service = ServiceProvider {};
        let response = links_service
            .search(Box::new(Arc::new(mock_links_repo)), &request_query)
            .await;

        assert!(response.is_ok());
        assert!(response.unwrap().is_empty());
    }

    #[rstest]
    #[case(true, "admin")]
    #[case(false, "user")]
    #[tokio::test]
    async fn test_search_links_non_empty(#[case] is_admin: bool, #[case] user: &str) {
        let request_query = LinkQueryBuilder::default()
            .user(user)
            .is_from_admin(is_admin)
            .build();
        let find_query = if is_admin {
            LinkQueryBuilder::default().user("").build()
        } else {
            LinkQueryBuilder::default().user(user).build()
        };

        let item = LinkItemBuilder::new("http://link")
            .id("1")
            .owner("user")
            .build();
        let expected_items = vec![item.clone()];

        let mut mock_links_repo = MockLinksRepo::new();
        mock_links_repo
            .expect_find()
            .withf(move |query| query == &find_query)
            .times(1)
            .returning(move |_| Ok(vec![item.clone()]));

        let links_service = ServiceProvider {};
        let response = links_service
            .search(Box::new(Arc::new(mock_links_repo)), &request_query)
            .await;

        assert!(response.is_ok());

        let returned_items = response.unwrap();
        assert!(!returned_items.is_empty());
        assert!(returned_items
            .iter()
            .all(|item| expected_items.contains(item)));
    }

    #[rstest]
    #[case(true, "admin")]
    #[case(false, "user")]
    #[tokio::test]
    async fn test_search_links_repo_error(#[case] is_admin: bool, #[case] user: &str) {
        let request_query = LinkQueryBuilder::default()
            .user(user)
            .is_from_admin(is_admin)
            .build();
        let find_query = if is_admin {
            LinkQueryBuilder::default().user("").build()
        } else {
            LinkQueryBuilder::default().user(user).build()
        };

        let mut mock_links_repo = MockLinksRepo::new();
        mock_links_repo
            .expect_find()
            .withf(move |query| query == &find_query)
            .times(1)
            .returning(|_| Err(AppError::Test));

        let links_service = ServiceProvider {};
        let response = links_service
            .search(Box::new(Arc::new(mock_links_repo)), &request_query)
            .await;

        assert_eq!(response, Err(AppError::Test));
    }

    #[rstest]
    #[case(true, "admin")]
    #[case(false, "user")]
    #[tokio::test]
    async fn test_get_link(#[case] is_admin: bool, #[case] user: &str) {
        let request_query = LinkQueryBuilder::new("1", user)
            .is_from_admin(is_admin)
            .build();
        let get_query = LinkQueryBuilder::default().id("1").build();
        let retrieved_item = LinkItemBuilder::new("http://link")
            .id("1")
            .owner("user")
            .build();
        let response_item = LinkItemBuilder::new("http://link")
            .id("1")
            .owner("user")
            .build();

        let mut mock_links_repo = MockLinksRepo::new();
        mock_links_repo
            .expect_get()
            .withf(move |query| query == &get_query)
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
    async fn test_get_link_unauthorized() {
        let request_query = LinkQueryBuilder::new("1", "unauthorized-user").build();
        let get_query = LinkQueryBuilder::default().id("1").build();
        let retrieved_item = LinkItemBuilder::new("http://link")
            .id("1")
            .owner("user")
            .build();

        let mut mock_links_repo = MockLinksRepo::new();
        mock_links_repo
            .expect_get()
            .withf(move |query| query == &get_query)
            .times(1)
            .returning(move |_| Ok(retrieved_item.clone()));

        let links_service = ServiceProvider {};
        let response = links_service
            .get(Box::new(Arc::new(mock_links_repo)), &request_query)
            .await;

        assert_eq!(
            response,
            Err(AppError::Authorization(
                "User is not authorized to access resource".into()
            ))
        );
    }

    #[rstest]
    #[case(true, "admin")]
    #[case(false, "user")]
    #[tokio::test]
    async fn test_get_link_not_found(#[case] is_admin: bool, #[case] user: &str) {
        let request_query = LinkQueryBuilder::new("1", user)
            .is_from_admin(is_admin)
            .build();
        let get_query = LinkQueryBuilder::default().id("1").build();

        let mut mock_links_repo = MockLinksRepo::new();
        mock_links_repo
            .expect_get()
            .withf(move |query| query == &get_query)
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
        let request_item = LinkItemBuilder::new("http://link").owner("user").build();
        let response_item = LinkItemBuilder::new("http://link")
            .id("1")
            .owner("user")
            .build();
        let item_to_create = request_item.clone();
        let created_item = response_item.clone();
        let item_to_analyze = created_item.clone();

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
        let request_item = LinkItemBuilder::new("http://link").owner("user").build();
        let item_to_create = request_item.clone();

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
        let request_item = LinkItemBuilder::new("http://link").owner("user").build();
        let response_item = LinkItemBuilder::new("http://link")
            .id("1")
            .owner("user")
            .build();
        let item_to_create = request_item.clone();
        let created_item = response_item.clone();
        let item_to_analyze = created_item.clone();

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

    #[rstest]
    #[case(true, "admin")]
    #[case(false, "user")]
    #[tokio::test]
    async fn test_update_link(#[case] is_admin: bool, #[case] user: &str) {
        let request_query = LinkQueryBuilder::new("1", user)
            .is_from_admin(is_admin)
            .build();
        let request_item = LinkItemBuilder::new("http://link")
            .description("sample link")
            .build();
        let response_item = LinkItemBuilder::new("http://link")
            .id("1")
            .owner("user")
            .description("sample link")
            .build();
        let get_query = LinkQueryBuilder::default().id("1").build();
        let retrieved_item = LinkItemBuilder::new("http://link")
            .id("1")
            .owner("user")
            .build();
        let update_query = LinkQueryBuilder::default().id("1").build();
        let item_to_update = LinkItemBuilder::new("http://link")
            .owner("user")
            .description("sample link")
            .build();
        let updated_item = response_item.clone();

        let mut seq = Sequence::new();

        let mut mock_links_repo = MockLinksRepo::new();
        mock_links_repo
            .expect_get()
            .withf(move |query| query == &get_query)
            .times(1)
            .in_sequence(&mut seq)
            .returning(move |_| Ok(retrieved_item.clone()));
        mock_links_repo
            .expect_update()
            .withf(move |query, item| {
                query == &update_query
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
                &request_query,
                &request_item,
            )
            .await;

        assert!(response.is_ok());
        assert_eq!(response.unwrap(), response_item);
    }

    #[rstest]
    #[case(true, "admin")]
    #[case(false, "user")]
    #[tokio::test]
    async fn test_update_link_update_url(#[case] is_admin: bool, #[case] user: &str) {
        let request_query = LinkQueryBuilder::new("1", user)
            .is_from_admin(is_admin)
            .build();
        let request_item = LinkItemBuilder::new("http://updated-link").build();
        let response_item = LinkItemBuilder::new("http://updated-link")
            .id("1")
            .owner("user")
            .build();
        let get_query = LinkQueryBuilder::default().id("1").build();
        let retrieved_item = LinkItemBuilder::new("http://link")
            .id("1")
            .owner("user")
            .build();
        let update_query = LinkQueryBuilder::default().id("1").build();
        let item_to_update = LinkItemBuilder::new("http://updated-link")
            .owner("user")
            .build();
        let updated_item = response_item.clone();
        let item_to_analyze = updated_item.clone();

        let mut seq = Sequence::new();

        let mut mock_links_repo = MockLinksRepo::new();
        mock_links_repo
            .expect_get()
            .withf(move |query| query == &get_query)
            .times(1)
            .in_sequence(&mut seq)
            .returning(move |_| Ok(retrieved_item.clone()));
        mock_links_repo
            .expect_update()
            .withf(move |query, item| {
                query == &update_query
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
                &request_query,
                &request_item,
            )
            .await;

        assert!(response.is_ok());
        assert_eq!(response.unwrap(), response_item);
    }

    #[tokio::test]
    async fn test_update_link_unauthorized() {
        let request_query = LinkQueryBuilder::new("1", "unauthorized-user").build();
        let request_item = LinkItemBuilder::new("http://link")
            .owner("unauthorized-user")
            .description("sample link")
            .build();
        let get_query = LinkQueryBuilder::default().id("1").build();
        let retrieved_item = LinkItemBuilder::new("http://link")
            .id("1")
            .owner("user")
            .build();

        let mut mock_links_repo = MockLinksRepo::new();
        mock_links_repo
            .expect_get()
            .withf(move |query| query == &get_query)
            .times(1)
            .returning(move |_| Ok(retrieved_item.clone()));
        mock_links_repo.expect_update().times(0);

        let mut mock_analysis_service = MockAnalysisService::new();
        mock_analysis_service.expect_analyze().times(0);

        let links_service = ServiceProvider {};
        let response = links_service
            .update(
                Box::new(Arc::new(mock_analysis_service)),
                Box::new(Arc::new(mock_links_repo)),
                &request_query,
                &request_item,
            )
            .await;

        assert_eq!(
            response,
            Err(AppError::Authorization(
                "User is not authorized to access resource".into()
            ))
        );
    }

    #[rstest]
    #[case(true, "admin")]
    #[case(false, "user")]
    #[tokio::test]
    async fn test_update_link_not_found(#[case] is_admin: bool, #[case] user: &str) {
        let request_query = LinkQueryBuilder::new("1", user)
            .is_from_admin(is_admin)
            .build();
        let request_item = LinkItemBuilder::new("http://link")
            .owner("user")
            .description("sample link")
            .build();
        let get_query = LinkQueryBuilder::default().id("1").build();

        let mut mock_links_repo = MockLinksRepo::new();
        mock_links_repo
            .expect_get()
            .withf(move |query| query == &get_query)
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
                &request_query,
                &request_item,
            )
            .await;

        assert_eq!(response, Err(AppError::LinkNotFound("1".into())));
    }

    #[rstest]
    #[case(true, "admin")]
    #[case(false, "user")]
    #[tokio::test]
    async fn test_update_link_repo_error(#[case] is_admin: bool, #[case] user: &str) {
        let request_query = LinkQueryBuilder::new("1", user)
            .is_from_admin(is_admin)
            .build();
        let request_item = LinkItemBuilder::new("http://link")
            .owner("user")
            .description("sample link")
            .build();
        let get_query = LinkQueryBuilder::default().id("1").build();
        let retrieved_item = LinkItemBuilder::new("http://link")
            .id("1")
            .owner("user")
            .build();
        let update_query = LinkQueryBuilder::default().id("1").build();
        let item_to_update = request_item.clone();

        let mut seq = Sequence::new();

        let mut mock_links_repo = MockLinksRepo::new();
        mock_links_repo
            .expect_get()
            .withf(move |query| query == &get_query)
            .times(1)
            .in_sequence(&mut seq)
            .returning(move |_| Ok(retrieved_item.clone()));
        mock_links_repo
            .expect_update()
            .withf(move |query, item| {
                query == &update_query
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
                &request_query,
                &request_item,
            )
            .await;

        assert_eq!(response, Err(AppError::Test));
    }

    #[rstest]
    #[case(true, "admin")]
    #[case(false, "user")]
    #[tokio::test]
    async fn test_update_link_analyze_error(#[case] is_admin: bool, #[case] user: &str) {
        let request_query = LinkQueryBuilder::new("1", user)
            .is_from_admin(is_admin)
            .build();
        let request_item = LinkItemBuilder::new("http://updated-link")
            .owner("user")
            .build();
        let get_query = LinkQueryBuilder::default().id("1").build();
        let retrieved_item = LinkItemBuilder::new("http://link")
            .id("1")
            .owner("user")
            .build();
        let update_query = LinkQueryBuilder::default().id("1").build();
        let item_to_update = LinkItemBuilder::new("http://updated-link")
            .owner("user")
            .build();
        let updated_item = LinkItemBuilder::new("http://updated-link")
            .id("1")
            .owner("user")
            .build();
        let item_to_analyze = updated_item.clone();

        let mut seq = Sequence::new();

        let mut mock_links_repo = MockLinksRepo::new();
        mock_links_repo
            .expect_get()
            .withf(move |query| query == &get_query)
            .times(1)
            .in_sequence(&mut seq)
            .returning(move |_| Ok(retrieved_item.clone()));
        mock_links_repo
            .expect_update()
            .withf(move |query, item| {
                query == &update_query
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
                &request_query,
                &request_item,
            )
            .await;

        assert_eq!(response, Err(AppError::Test));
    }

    #[rstest]
    #[case(true, "admin")]
    #[case(false, "user")]
    #[tokio::test]
    async fn test_delete_link(#[case] is_admin: bool, #[case] user: &str) {
        let request_query = LinkQueryBuilder::new("1", user)
            .is_from_admin(is_admin)
            .build();
        let get_query = LinkQueryBuilder::default().id("1").build();
        let retrieved_item = LinkItemBuilder::new("http://link")
            .id("1")
            .owner("user")
            .build();
        let delete_query = LinkQueryBuilder::default().id("1").build();

        let mut seq = Sequence::new();

        let mut mock_links_repo = MockLinksRepo::new();
        mock_links_repo
            .expect_get()
            .withf(move |query| query == &get_query)
            .times(1)
            .in_sequence(&mut seq)
            .returning(move |_| Ok(retrieved_item.clone()));
        mock_links_repo
            .expect_delete()
            .withf(move |query| query == &delete_query)
            .times(1)
            .in_sequence(&mut seq)
            .returning(move |_| Ok(()));

        let links_service = ServiceProvider {};
        let response = links_service
            .delete(Box::new(Arc::new(mock_links_repo)), &request_query)
            .await;

        assert!(response.is_ok());
    }

    #[tokio::test]
    async fn test_delete_link_unauthorized() {
        let request_query = LinkQueryBuilder::new("1", "unauthorized-user").build();
        let get_query = LinkQueryBuilder::default().id("1").build();
        let retrieved_item = LinkItemBuilder::new("http://link")
            .id("1")
            .owner("user")
            .build();

        let mut mock_links_repo = MockLinksRepo::new();
        mock_links_repo
            .expect_get()
            .withf(move |query| query == &get_query)
            .times(1)
            .returning(move |_| Ok(retrieved_item.clone()));
        mock_links_repo.expect_delete().times(0);

        let links_service = ServiceProvider {};
        let response = links_service
            .delete(Box::new(Arc::new(mock_links_repo)), &request_query)
            .await;

        assert_eq!(
            response,
            Err(AppError::Authorization(
                "User is not authorized to access resource".into()
            ))
        );
    }

    #[rstest]
    #[case(true, "admin")]
    #[case(false, "user")]
    #[tokio::test]
    async fn test_delete_link_not_found(#[case] is_admin: bool, #[case] user: &str) {
        let request_query = LinkQueryBuilder::new("1", user)
            .is_from_admin(is_admin)
            .build();
        let get_query = LinkQueryBuilder::default().id("1").build();

        let mut mock_links_repo = MockLinksRepo::new();
        mock_links_repo
            .expect_get()
            .withf(move |query| query == &get_query)
            .times(1)
            .returning(|_| Err(AppError::LinkNotFound("1".into())));
        mock_links_repo.expect_delete().times(0);

        let links_service = ServiceProvider {};
        let response = links_service
            .delete(Box::new(Arc::new(mock_links_repo)), &request_query)
            .await;

        assert_eq!(response, Err(AppError::LinkNotFound("1".into())));
    }

    #[rstest]
    #[case(true, "admin")]
    #[case(false, "user")]
    #[tokio::test]
    async fn test_delete_link_repo_error(#[case] is_admin: bool, #[case] user: &str) {
        let request_query = LinkQueryBuilder::new("1", user)
            .is_from_admin(is_admin)
            .build();
        let get_query = LinkQueryBuilder::default().id("1").build();
        let retrieved_item = LinkItemBuilder::new("http://link")
            .id("1")
            .owner("user")
            .build();
        let delete_query = LinkQueryBuilder::default().id("1").build();

        let mut seq = Sequence::new();

        let mut mock_links_repo = MockLinksRepo::new();
        mock_links_repo
            .expect_get()
            .withf(move |query| query == &get_query)
            .times(1)
            .in_sequence(&mut seq)
            .returning(move |_| Ok(retrieved_item.clone()));
        mock_links_repo
            .expect_delete()
            .withf(move |query| query == &delete_query)
            .times(1)
            .in_sequence(&mut seq)
            .returning(|_| Err(AppError::Test));

        let links_service = ServiceProvider {};
        let response = links_service
            .delete(Box::new(Arc::new(mock_links_repo)), &request_query)
            .await;

        assert_eq!(response, Err(AppError::Test));
    }
}
