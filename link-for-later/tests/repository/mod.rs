use axum::async_trait;

use link_for_later_types::entity::{LinkItem, UserInfo};

pub mod inmemory;
pub mod mongodb;

#[derive(Clone)]
pub enum DatabaseType {
    MongoDb,
    InMemory,
}

#[async_trait]
pub trait Repository {
    async fn count_links(&self) -> u64;
    async fn get_link(&self, id: &str) -> LinkItem;
    async fn add_link(&self, owner: &str, url: &str) -> String;
    async fn count_users(&self) -> u64;
    async fn get_user(&self, email: &str) -> UserInfo;
    async fn add_user(&self, email: &str, password: &str) -> String;
}

pub fn new(db_type: &DatabaseType) -> Box<dyn Repository> {
    match db_type {
        DatabaseType::InMemory => Box::<inmemory::RepositoryProvider>::default(),
        DatabaseType::MongoDb => {
            let repository = mongodb::RepositoryProvider::default();
            repository.setup();
            Box::new(repository)
        }
    }
}
