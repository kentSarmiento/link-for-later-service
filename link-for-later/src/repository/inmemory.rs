use std::sync::Mutex;

use axum::async_trait;

use crate::types::{
    AppError, LinkItem, LinkItemBuilder, LinkQuery, Result, UserInfo, UserInfoBuilder, UserQuery,
};

use super::{Links as LinksRepository, Users as UsersRepository};

pub struct LinksRepositoryProvider {
    links_data: Mutex<Vec<LinkItem>>,
    links_data_counter: Mutex<Vec<usize>>,
}

pub struct UsersRepositoryProvider {
    users_data: Mutex<Vec<UserInfo>>,
    users_data_counter: Mutex<Vec<usize>>,
}

impl Default for LinksRepositoryProvider {
    fn default() -> Self {
        Self {
            links_data: Mutex::new(Vec::new()),
            links_data_counter: Mutex::new(Vec::new()),
        }
    }
}

impl Default for UsersRepositoryProvider {
    fn default() -> Self {
        Self {
            users_data: Mutex::new(Vec::new()),
            users_data_counter: Mutex::new(Vec::new()),
        }
    }
}

#[async_trait]
impl LinksRepository for LinksRepositoryProvider {
    async fn find(&self, query: &LinkQuery) -> Result<Vec<LinkItem>> {
        let filtered_links: Vec<LinkItem> = self
            .links_data
            .lock()
            .map_err(|e| AppError::Database(format!("find() {e:?}")))?
            .iter()
            .filter(|link| {
                (link.id() == query.id() || query.id().is_empty())
                    && (link.owner() == query.user() || query.user().is_empty())
            })
            .cloned()
            .collect();
        Ok(filtered_links)
    }

    async fn get(&self, query: &LinkQuery) -> Result<LinkItem> {
        self.links_data
            .lock()
            .map_err(|e| AppError::Database(format!("get() {e:?}")))?
            .iter()
            .find(|link| {
                link.id() == query.id() && (link.owner() == query.user() || query.user().is_empty())
            })
            .cloned()
            .ok_or_else(|| AppError::LinkNotFound(query.id().to_owned()))
    }

    async fn create(&self, item: &LinkItem) -> Result<LinkItem> {
        let id = self
            .links_data_counter
            .lock()
            .map_err(|e| AppError::Database(format!("create() {e:?}")))?
            .len()
            + 1;
        let link = LinkItemBuilder::from(item.clone())
            .id(&id.to_string())
            .build();
        self.links_data
            .lock()
            .map_err(|e| AppError::Database(format!("create() {e:?}")))?
            .push(link.clone());
        self.links_data_counter
            .lock()
            .map_err(|e| AppError::Database(format!("create() {e:?}")))?
            .push(id);
        Ok(link)
    }

    async fn update(&self, query: &LinkQuery, item: &LinkItem) -> Result<LinkItem> {
        self.links_data
            .lock()
            .map_err(|e| AppError::Database(format!("update() {e:?}")))?
            .iter()
            .find(|link| link.id() == query.id() && link.owner() == item.owner())
            .cloned()
            .ok_or_else(|| AppError::LinkNotFound(query.id().to_owned()))?;
        self.delete(query).await?;
        self.links_data
            .lock()
            .map_err(|e| AppError::Database(format!("update() {e:?}")))?
            .push(item.clone());
        Ok(item.clone())
    }

    async fn delete(&self, query: &LinkQuery) -> Result<()> {
        self.get(query).await?;
        self.links_data
            .lock()
            .map_err(|e| AppError::Database(format!("delete() {e:?}")))?
            .retain(|link| link.id() != query.id());
        Ok(())
    }
}

#[async_trait]
impl UsersRepository for UsersRepositoryProvider {
    async fn get(&self, query: &UserQuery) -> Result<UserInfo> {
        self.users_data
            .lock()
            .map_err(|e| AppError::Database(format!("get() {e:?}")))?
            .iter()
            .find(|user| user.email() == query.email())
            .cloned()
            .ok_or_else(|| AppError::UserNotFound(query.email().to_owned()))
    }

    async fn create(&self, info: &UserInfo) -> Result<UserInfo> {
        let id = self
            .users_data_counter
            .lock()
            .map_err(|e| AppError::Database(format!("create() {e:?}")))?
            .len()
            + 1;
        let user = UserInfoBuilder::from(info.clone())
            .id(&id.to_string())
            .build();
        self.users_data
            .lock()
            .map_err(|e| AppError::Database(format!("create() {e:?}")))?
            .push(user.clone());
        self.users_data_counter
            .lock()
            .map_err(|e| AppError::Database(format!("create() {e:?}")))?
            .push(id);
        Ok(user)
    }
}

#[cfg(test)]
mod tests {

    use crate::types::{LinkQueryBuilder, UserQueryBuilder};

    use super::*;

    #[tokio::test]
    async fn test_search_links_empty() {
        let repo_query = LinkQueryBuilder::default().user("user-id").build();
        let links_repository = LinksRepositoryProvider::default();

        let retrieved_items = links_repository.find(&repo_query).await.unwrap();
        assert!(retrieved_items.is_empty());
    }

    #[tokio::test]
    async fn test_search_created_links() {
        let item = LinkItemBuilder::new("http://link").owner("user-id").build();

        let links_repository = LinksRepositoryProvider::default();
        let created_item = links_repository.create(&item).await.unwrap();
        let expected_items = vec![created_item.clone()];

        let repo_query = LinkQueryBuilder::default().user("user-id").build();
        let retrieved_items = links_repository.find(&repo_query).await.unwrap();
        assert!(!retrieved_items.is_empty());
        assert!(retrieved_items
            .iter()
            .all(|item| expected_items.contains(item)));
    }

    #[tokio::test]
    async fn test_get_link_not_found() {
        let repo_query = LinkQueryBuilder::new("1", "user-id").build();

        let links_repository = LinksRepositoryProvider::default();
        let response = links_repository.get(&repo_query).await;

        assert_eq!(response, Err(AppError::LinkNotFound("1".into())));
    }

    #[tokio::test]
    async fn test_get_created_link() {
        let item = LinkItemBuilder::new("http://link").owner("user-id").build();

        let links_repository = LinksRepositoryProvider::default();
        let created_item = links_repository.create(&item).await.unwrap();

        let repo_query = LinkQueryBuilder::new(created_item.id(), "user-id").build();
        let retrieved_item = links_repository.get(&repo_query).await.unwrap();

        assert_eq!(created_item, retrieved_item);
    }

    #[tokio::test]
    async fn test_update_link_not_found() {
        let repo_query = LinkQueryBuilder::new("1", "user-id").build();
        let item = LinkItemBuilder::new("http://link").owner("user-id").build();

        let links_repository = LinksRepositoryProvider::default();
        let response = links_repository.update(&repo_query, &item).await;

        assert_eq!(response, Err(AppError::LinkNotFound("1".into())));
    }

    #[tokio::test]
    async fn test_update_created_link() {
        let item = LinkItemBuilder::new("http://link").owner("user-id").build();

        let links_repository = LinksRepositoryProvider::default();
        let created_item = links_repository.create(&item).await.unwrap();

        let repo_query = LinkQueryBuilder::new(created_item.id(), "user-id").build();
        let item = LinkItemBuilder::from(created_item.clone())
            .title("title")
            .build();
        let updated_item = links_repository.update(&repo_query, &item).await.unwrap();

        let repo_query = LinkQueryBuilder::new(updated_item.id(), "user-id").build();
        let retrieved_item = links_repository.get(&repo_query).await.unwrap();

        assert_eq!(updated_item, retrieved_item);
    }

    #[tokio::test]
    async fn test_delete_link_not_found() {
        let repo_query = LinkQueryBuilder::new("1", "user-id").build();

        let links_repository = LinksRepositoryProvider::default();
        let response = links_repository.delete(&repo_query).await;

        assert_eq!(response, Err(AppError::LinkNotFound("1".into())));
    }

    #[tokio::test]
    async fn test_delete_created_link() {
        let item = LinkItemBuilder::new("http://link").owner("user-id").build();

        let links_repository = LinksRepositoryProvider::default();
        let created_item = links_repository.create(&item).await.unwrap();

        let repo_query = LinkQueryBuilder::new(created_item.id(), "user-id").build();
        links_repository.delete(&repo_query).await.unwrap();

        let repo_query = LinkQueryBuilder::new(created_item.id(), "user-id").build();
        let response = links_repository.get(&repo_query).await;

        assert_eq!(response, Err(AppError::LinkNotFound("1".into())));
    }

    #[tokio::test]
    async fn test_get_user_not_found() {
        let repo_query = UserQueryBuilder::new("user@test.com").build();

        let users_repository = UsersRepositoryProvider::default();
        let response = users_repository.get(&repo_query).await;

        assert_eq!(
            response,
            Err(AppError::UserNotFound("user@test.com".into()))
        );
    }

    #[tokio::test]
    async fn test_get_created_user() {
        let user = UserInfoBuilder::new("user@test.com", "test").build();

        let users_repository = UsersRepositoryProvider::default();
        let created_user = users_repository.create(&user).await.unwrap();

        let repo_query = UserQueryBuilder::new("user@test.com").build();
        let retrieved_user = users_repository.get(&repo_query).await.unwrap();

        assert_eq!(created_user, retrieved_user);
    }
}
