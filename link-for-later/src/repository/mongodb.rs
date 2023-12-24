use std::str::FromStr;

use axum::async_trait;
use bson::{doc, Bson};
use futures::TryStreamExt;
use mongodb::{options::ReplaceOptions, Collection, Database};

use crate::types::{
    dto::{LinkQuery, UserQuery},
    entity::LinkItem,
    entity::{LinkItemBuilder, UserInfo, UserInfoBuilder},
    AppError, Result,
};

use super::{Links as LinksRepository, Users as UsersRepository};

const LINKS_COLLECTION_NAME: &str = "v0.1.2/links";
const USERS_COLLECTION_NAME: &str = "v0.1.2/users";

pub struct LinksDb {
    links_collection: Collection<LinkItem>,
}

pub struct UsersDb {
    users_collection: Collection<UserInfo>,
}

impl LinksDb {
    pub fn new(db: &Database) -> Self {
        let collection_name = std::env::var("LINKS_COLLECTION_NAME")
            .unwrap_or_else(|_| LINKS_COLLECTION_NAME.to_string());
        let links_collection = db.collection::<LinkItem>(&collection_name);
        Self { links_collection }
    }
}

impl UsersDb {
    pub fn new(db: &Database) -> Self {
        let collection_name = std::env::var("USERS_COLLECTION_NAME")
            .unwrap_or_else(|_| USERS_COLLECTION_NAME.to_string());
        let users_collection = db.collection::<UserInfo>(&collection_name);
        Self { users_collection }
    }
}

#[async_trait]
impl LinksRepository for LinksDb {
    async fn find(&self, query: &LinkQuery) -> Result<Vec<LinkItem>> {
        let mut db_query = doc! {};
        if !query.id().is_empty() {
            let Ok(id) = bson::oid::ObjectId::from_str(query.id()) else {
                tracing::error!("Error: {} cannot be converted to Bson ObjectId", query.id());
                return Err(AppError::LinkNotFound);
            };
            db_query.insert("_id", id);
        }
        if !query.owner().is_empty() {
            db_query.insert("owner", query.owner());
        }
        match self.links_collection.find(db_query, None).await {
            Ok(result) => Ok(result.try_collect().await.unwrap_or_else(|_| vec![])),
            Err(e) => {
                tracing::error!("Error: find(): {e:?}");
                Err(AppError::DatabaseError)
            }
        }
    }

    async fn get(&self, query: &LinkQuery) -> Result<LinkItem> {
        let mut db_query = doc! {};
        if !query.id().is_empty() {
            let Ok(id) = bson::oid::ObjectId::from_str(query.id()) else {
                tracing::error!("Error: {} cannot be converted to Bson ObjectId", query.id());
                return Err(AppError::LinkNotFound);
            };
            db_query.insert("_id", id);
        }
        if !query.owner().is_empty() {
            db_query.insert("owner", query.owner());
        }
        match self.links_collection.find_one(db_query, None).await {
            Ok(item) => item.map_or(Err(AppError::LinkNotFound), |item| {
                let returned_item = LinkItemBuilder::from(item).id(query.id()).build();
                Ok(returned_item)
            }),
            Err(e) => {
                tracing::error!("Error: find_one(): {e:?}");
                Err(AppError::DatabaseError)
            }
        }
    }

    async fn create(&self, item: &LinkItem) -> Result<LinkItem> {
        match self.links_collection.insert_one(item, None).await {
            Ok(result) => {
                let id = if let Bson::ObjectId(id) = result.inserted_id {
                    id.to_hex()
                } else {
                    tracing::error!("Error: unexpected inserted_id: {}", result.inserted_id);
                    return Err(AppError::DatabaseError);
                };
                let query = doc! {"_id": result.inserted_id};
                let update = doc! {"$set": doc! { "id": &id } };
                self.links_collection
                    .update_one(query, update, None)
                    .await
                    .unwrap();

                let returned_item = LinkItemBuilder::from(item.clone()).id(&id).build();
                Ok(returned_item)
            }
            Err(e) => {
                tracing::error!("Error: insert_one(): {e:?}");
                Err(AppError::DatabaseError)
            }
        }
    }

    async fn update(&self, id: &str, item: &LinkItem) -> Result<LinkItem> {
        let Ok(id) = bson::oid::ObjectId::from_str(id) else {
            tracing::error!("Error: {id} cannot be converted to Bson ObjectId");
            return Err(AppError::LinkNotFound);
        };
        let query = doc! {"_id": id, "owner": item.owner()};
        let opts = ReplaceOptions::builder().upsert(true).build();
        match self
            .links_collection
            .replace_one(query, item, Some(opts))
            .await
        {
            Ok(_) => Ok(item.clone()),
            Err(e) => {
                tracing::error!("Error: replace_one(): {e:?}");
                Err(AppError::DatabaseError)
            }
        }
    }

    async fn delete(&self, item: &LinkItem) -> Result<()> {
        let Ok(id) = bson::oid::ObjectId::from_str(item.id()) else {
            tracing::error!("Error: {} cannot be converted to Bson ObjectId", item.id());
            return Err(AppError::LinkNotFound);
        };
        let query = doc! {"_id": id, "owner": item.owner()};
        match self.links_collection.delete_one(query, None).await {
            Ok(_) => Ok(()),
            Err(e) => {
                tracing::error!("Error: delete_one(): {e:?}");
                Err(AppError::DatabaseError)
            }
        }
    }
}

#[async_trait]
impl UsersRepository for UsersDb {
    async fn get(&self, query: &UserQuery) -> Result<UserInfo> {
        let query = doc! {"email": query.email()};
        match self.users_collection.find_one(query, None).await {
            Ok(item) => item.ok_or(AppError::UserNotFound),
            Err(e) => {
                tracing::error!("Error: find_one(): {e:?}");
                Err(AppError::DatabaseError)
            }
        }
    }

    async fn create(&self, info: &UserInfo) -> Result<UserInfo> {
        match self.users_collection.insert_one(info, None).await {
            Ok(result) => {
                let id = if let Bson::ObjectId(id) = result.inserted_id {
                    id.to_hex()
                } else {
                    tracing::error!("Error: unexpected inserted_id: {}", result.inserted_id);
                    return Err(AppError::DatabaseError);
                };
                let query = doc! {"_id": result.inserted_id};
                let update = doc! {"$set": doc! { "id": &id } };
                self.users_collection
                    .update_one(query, update, None)
                    .await
                    .unwrap();
                let returned_info = UserInfoBuilder::from(info.clone()).id(&id).build();
                Ok(returned_info)
            }
            Err(e) => {
                tracing::error!("Error: insert_one(): {e:?}");
                Err(AppError::DatabaseError)
            }
        }
    }
}
