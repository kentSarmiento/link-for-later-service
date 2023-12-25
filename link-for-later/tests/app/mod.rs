#![allow(dead_code)]
use axum::Router;

use crate::repository::{self, DatabaseType};

pub async fn new(db_type: &DatabaseType) -> Router {
    match db_type {
        DatabaseType::InMemory => link_for_later::app::new(link_for_later::DatabaseType::InMemory),
        DatabaseType::MongoDb => {
            let db = repository::mongodb::database().await;
            link_for_later::app::new(link_for_later::DatabaseType::MongoDb(db))
        }
    }
}
