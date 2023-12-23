#![allow(dead_code)]
use axum::Router;

use crate::repository;

pub async fn new() -> Router {
    let db = repository::database().await;
    link_for_later::app::new(link_for_later::DatabaseType::MongoDb(db))
}
