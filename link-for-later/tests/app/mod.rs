use axum::Router;

use crate::repository::{mongodb, DatabaseType};

pub async fn new(db_type: &DatabaseType) -> Router {
    match db_type {
        DatabaseType::InMemory => link_for_later::app::new(link_for_later::DatabaseType::InMemory),
        DatabaseType::MongoDb => {
            let db = mongodb::database().await;
            link_for_later::app::new(link_for_later::DatabaseType::MongoDb(db))
        }
    }
}
