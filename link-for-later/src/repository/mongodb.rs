use std::str::FromStr;

use axum::async_trait;
use bson::{doc, Bson};
use futures::TryStreamExt;
use mongodb::{options::ReplaceOptions, Collection, Database};

use crate::types::{links::LinkItem, repository::Links, AppError, Result};

const LINKS_COLLECTION_NAME: &str = "v1/links";

pub struct MongoDbRepository {
    collection: Collection<LinkItem>,
}

impl MongoDbRepository {
    pub fn new(db: &Database) -> Self {
        let collection = db.collection::<LinkItem>(LINKS_COLLECTION_NAME);
        Self { collection }
    }
}

#[async_trait]
impl Links for MongoDbRepository {
    async fn list(&self) -> Result<Vec<LinkItem>> {
        match self.collection.find(None, None).await {
            Ok(result) => Ok(result.try_collect().await.unwrap_or_else(|_| vec![])),
            Err(e) => {
                tracing::error!("Error: find(): {e:?}");
                Err(AppError::DatabaseError)
            }
        }
    }

    async fn post(&self, item: &LinkItem) -> Result<LinkItem> {
        match self.collection.insert_one(item, None).await {
            Ok(result) => {
                let id = if let Bson::ObjectId(id) = result.inserted_id {
                    id.to_hex()
                } else {
                    tracing::error!("Error: unexpected inserted_id: {}", result.inserted_id);
                    return Err(AppError::DatabaseError);
                };
                let query = doc! {"_id": result.inserted_id};
                let update = doc! {"$set": doc! { "id": &id } };
                self.collection
                    .update_one(query, update, None)
                    .await
                    .unwrap();
                let returned_item = item.clone().id(&id);
                Ok(returned_item)
            }
            Err(e) => {
                tracing::error!("Error: insert_one(): {e:?}");
                Err(AppError::DatabaseError)
            }
        }
    }

    async fn get(&self, id: &str) -> Result<LinkItem> {
        let Ok(oid) = bson::oid::ObjectId::from_str(id) else {
            tracing::error!("Error: {id} cannot be converted to Bson ObjectId");
            return Err(AppError::ItemNotFound);
        };
        let query = doc! {"_id": oid};
        match self.collection.find_one(query, None).await {
            Ok(item) => item.map_or(Err(AppError::ItemNotFound), |item| {
                let returned_item = item.id(id);
                Ok(returned_item)
            }),
            Err(e) => {
                tracing::error!("Error: find_one(): {e:?}");
                Err(AppError::DatabaseError)
            }
        }
    }

    async fn put(&self, id: &str, item: &LinkItem) -> Result<LinkItem> {
        let Ok(id) = bson::oid::ObjectId::from_str(id) else {
            tracing::error!("Error: {id} cannot be converted to Bson ObjectId");
            return Err(AppError::ItemNotFound);
        };
        let query = doc! {"_id": id};
        let opts = ReplaceOptions::builder().upsert(true).build();
        match self.collection.replace_one(query, item, Some(opts)).await {
            Ok(_) => Ok(item.clone()),
            Err(e) => {
                tracing::error!("Error: replace_one(): {e:?}");
                Err(AppError::DatabaseError)
            }
        }
    }

    async fn delete(&self, id: &str) -> Result<()> {
        let Ok(id) = bson::oid::ObjectId::from_str(id) else {
            tracing::error!("Error: {id} cannot be converted to Bson ObjectId");
            return Err(AppError::ItemNotFound);
        };
        let query = doc! {"_id": id};
        match self.collection.delete_one(query, None).await {
            Ok(_) => Ok(()),
            Err(e) => {
                tracing::error!("Error: delete_one(): {e:?}");
                Err(AppError::DatabaseError)
            }
        }
    }
}
