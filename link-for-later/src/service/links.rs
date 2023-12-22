use axum::async_trait;
use chrono::Utc;

use crate::{
    repository,
    service::Links as LinksService,
    types::{
        dto::{LinkQuery, LinkQueryBuilder},
        entity::{LinkItem, LinkItemBuilder},
        Result,
    },
};

pub struct ServiceProvider {}

#[async_trait]
impl LinksService for ServiceProvider {
    async fn search(
        &self,
        links_repo: Box<repository::DynLinks>,
        link_query: &LinkQuery,
    ) -> Result<Vec<LinkItem>> {
        links_repo.search(link_query).await
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
        links_repo: Box<repository::DynLinks>,
        link_item: &LinkItem,
    ) -> Result<LinkItem> {
        let now = Utc::now().to_rfc3339();
        let created_link_item = LinkItemBuilder::from(link_item.clone())
            .created_at(&now)
            .updated_at(&now)
            .build();

        links_repo.create(&created_link_item).await
    }

    async fn update(
        &self,
        links_repo: Box<repository::DynLinks>,
        id: &str,
        link_item: &LinkItem,
    ) -> Result<LinkItem> {
        let link_query = LinkQueryBuilder::new(link_item.id(), link_item.owner()).build();
        let retrieved_link_item = links_repo.get(&link_query).await?;

        let now = Utc::now().to_rfc3339();
        let updated_link_item = LinkItemBuilder::from(link_item.clone())
            .created_at(retrieved_link_item.created_at())
            .updated_at(&now)
            .build();

        links_repo.update(id, &updated_link_item).await
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
