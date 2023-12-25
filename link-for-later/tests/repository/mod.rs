#![allow(dead_code)]

use axum::async_trait;

use crate::entity::{LinkItem, UserInfo};

use self::{inmemory::Repository as InMemoryRepo, mongodb::Repository as MongoDbRepo};

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
        DatabaseType::InMemory => Box::<InMemoryRepo>::default(),
        DatabaseType::MongoDb => Box::<MongoDbRepo>::default(),
    }
}
