#![allow(dead_code)]

use crate::entity::{LinkItem, UserInfo};

pub mod mongodb;

#[derive(Clone)]
pub enum DatabaseType {
    MongoDb,
    InMemory,
}

pub struct Repository {
    db_type: DatabaseType,
}

pub fn new(db_type: &DatabaseType) -> Repository {
    let db_type = db_type.clone();
    match db_type {
        DatabaseType::InMemory => {}
        DatabaseType::MongoDb => mongodb::setup(),
    };
    Repository { db_type }
}

impl Repository {
    pub async fn count_links(&self) -> u64 {
        match self.db_type {
            DatabaseType::InMemory => unimplemented!(),
            DatabaseType::MongoDb => mongodb::count_links().await,
        }
    }

    pub async fn get_link(&self, id: &str) -> LinkItem {
        match self.db_type {
            DatabaseType::InMemory => unimplemented!(),
            DatabaseType::MongoDb => mongodb::get_link(id).await,
        }
    }

    pub async fn add_link(&self, owner: &str, url: &str) -> String {
        match self.db_type {
            DatabaseType::InMemory => unimplemented!(),
            DatabaseType::MongoDb => mongodb::add_link(owner, url).await,
        }
    }

    pub async fn count_users(&self) -> u64 {
        match self.db_type {
            DatabaseType::InMemory => unimplemented!(),
            DatabaseType::MongoDb => mongodb::count_users().await,
        }
    }

    pub async fn get_user(&self, email: &str) -> UserInfo {
        match self.db_type {
            DatabaseType::InMemory => unimplemented!(),
            DatabaseType::MongoDb => mongodb::get_user(email).await,
        }
    }

    pub async fn add_user(&self, email: &str, password: &str) -> String {
        match self.db_type {
            DatabaseType::InMemory => unimplemented!(),
            DatabaseType::MongoDb => mongodb::add_user(email, password).await,
        }
    }
}
