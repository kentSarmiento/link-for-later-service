use axum::async_trait;
use mongodb::{bson::doc, Collection, Database};

use crate::types::{links::LinkItem, repository::Links, AppError, Result};

const LINKS_COLLECTION_NAME: &str = "v1/links";

#[derive(Default)]
pub struct MongoDbRepository {
    collection: Option<Collection<LinkItem>>,
}

impl MongoDbRepository {
    pub fn new(db: &Database) -> Self {
        let collection = db.collection::<LinkItem>(LINKS_COLLECTION_NAME);
        Self {
            collection: Some(collection),
        }
    }
}

#[async_trait]
impl Links for MongoDbRepository {
    async fn list(&self) -> Result<Vec<LinkItem>> {
        Err(AppError::NotSupported)
    }

    async fn post(&self, item: &LinkItem) -> Result<LinkItem> {
        let Some(collection) = &self.collection else {
            return Err(AppError::NoDatabaseSetup);
        };
        match collection.insert_one(item, None).await {
            Ok(_) => Ok(item.clone()),
            Err(e) => {
                println!("insert_one() error: {e:?}");
                Err(AppError::DatabaseError)
            }
        }
    }

    async fn get(&self, id: &str) -> Result<LinkItem> {
        let Some(collection) = &self.collection else {
            return Err(AppError::NoDatabaseSetup);
        };
        let filter = doc! {"id": id};
        match collection.find_one(filter, None).await {
            Ok(item) => item.ok_or(AppError::ItemNotFound),
            Err(e) => {
                println!("find_one() error: {e:?}");
                Err(AppError::DatabaseError)
            }
        }
    }

    async fn put(&self, _id: &str) -> Result<LinkItem> {
        Err(AppError::NotSupported)
    }

    async fn delete(&self, id: &str) -> Result<()> {
        let Some(collection) = &self.collection else {
            return Err(AppError::NoDatabaseSetup);
        };
        let query = doc! {"id": id};
        match collection.delete_one(query, None).await {
            Ok(_) => Ok(()),
            Err(e) => {
                println!("delete_one() error: {e:?}");
                Err(AppError::DatabaseError)
            }
        }
    }
}
