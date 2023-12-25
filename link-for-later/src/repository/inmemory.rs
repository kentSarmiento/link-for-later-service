use std::sync::Mutex;

use axum::async_trait;
use once_cell::sync::Lazy;

use crate::types::{
    dto::{LinkQuery, LinkQueryBuilder, UserQuery},
    entity::{LinkItem, LinkItemBuilder, UserInfo, UserInfoBuilder},
    AppError, Result,
};

use super::{Links as LinksRepository, Users as UsersRepository};

#[derive(Default)]
pub struct LinksRepositoryProvider {}

static INMEMORY_LINKS_DATA: Lazy<Mutex<Vec<LinkItem>>> = Lazy::new(|| Mutex::new(Vec::new()));
static INMEMORY_LINKS_DATA_COUNTER: Lazy<Mutex<Vec<usize>>> = Lazy::new(|| Mutex::new(Vec::new()));

#[derive(Default)]
pub struct UsersRepositoryProvider {}

static INMEMORY_USERS_DATA: Lazy<Mutex<Vec<UserInfo>>> = Lazy::new(|| Mutex::new(Vec::new()));
static INMEMORY_USERS_DATA_COUNTER: Lazy<Mutex<Vec<usize>>> = Lazy::new(|| Mutex::new(Vec::new()));

#[async_trait]
impl LinksRepository for LinksRepositoryProvider {
    async fn find(&self, query: &LinkQuery) -> Result<Vec<LinkItem>> {
        let filtered_links: Vec<LinkItem> = INMEMORY_LINKS_DATA
            .lock()
            .unwrap()
            .iter()
            .filter(|link| {
                (link.id() == query.id() || query.id().is_empty())
                    && (link.owner() == query.owner() || query.owner().is_empty())
            })
            .cloned()
            .collect();
        Ok(filtered_links)
    }

    async fn get(&self, query: &LinkQuery) -> Result<LinkItem> {
        INMEMORY_LINKS_DATA
            .lock()
            .unwrap()
            .iter()
            .find(|link| link.id() == query.id() && link.owner() == query.owner())
            .cloned()
            .ok_or(AppError::LinkNotFound)
    }

    async fn create(&self, item: &LinkItem) -> Result<LinkItem> {
        let id = INMEMORY_LINKS_DATA_COUNTER.lock().unwrap().len() + 1;
        let link = LinkItemBuilder::from(item.clone())
            .id(&id.to_string())
            .build();
        INMEMORY_LINKS_DATA.lock().unwrap().push(link.clone());
        INMEMORY_LINKS_DATA_COUNTER.lock().unwrap().push(id);
        Ok(link)
    }

    async fn update(&self, id: &str, item: &LinkItem) -> Result<LinkItem> {
        INMEMORY_LINKS_DATA
            .lock()
            .unwrap()
            .iter()
            .find(|link| link.id() == id && link.owner() == item.owner())
            .cloned()
            .ok_or(AppError::LinkNotFound)?;
        self.delete(item).await?;
        INMEMORY_LINKS_DATA.lock().unwrap().push(item.clone());
        Ok(item.clone())
    }

    async fn delete(&self, item: &LinkItem) -> Result<()> {
        let query = LinkQueryBuilder::new(item.id(), item.owner()).build();
        self.get(&query).await?;
        INMEMORY_LINKS_DATA
            .lock()
            .unwrap()
            .retain(|link| link.id() != query.id());
        Ok(())
    }
}

#[async_trait]
impl UsersRepository for UsersRepositoryProvider {
    async fn get(&self, query: &UserQuery) -> Result<UserInfo> {
        INMEMORY_USERS_DATA
            .lock()
            .unwrap()
            .iter()
            .find(|user| user.email() == query.email())
            .cloned()
            .ok_or(AppError::UserNotFound)
    }

    async fn create(&self, info: &UserInfo) -> Result<UserInfo> {
        let id = INMEMORY_USERS_DATA_COUNTER.lock().unwrap().len() + 1;
        let user = UserInfoBuilder::from(info.clone())
            .id(&id.to_string())
            .build();
        INMEMORY_USERS_DATA.lock().unwrap().push(user.clone());
        INMEMORY_USERS_DATA_COUNTER.lock().unwrap().push(id);
        Ok(user)
    }
}
