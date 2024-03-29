use axum::async_trait;
use bson::{doc, to_document};
use futures::TryStreamExt;
use mongodb::{options::ReplaceOptions, Collection, Database};

use crate::types::{
    AppError, LinkItem, LinkItemBuilder, LinkQuery, Result, UserInfo, UserInfoBuilder, UserQuery,
};

use super::{Links as LinksRepository, Users as UsersRepository};

const LINKS_COLLECTION_NAME_KEY: &str = "LINKS_COLLECTION_NAME";
const LINKS_COLLECTION_NAME_DEFAULT: &str = "v1/links";

const USERS_COLLECTION_NAME_KEY: &str = "USERS_COLLECTION_NAME";
const USERS_COLLECTION_NAME_DEFAULT: &str = "v1/users";

pub struct LinksRepositoryProvider {
    links_collection: Collection<LinkItem>,
}

pub struct UsersRepositoryProvider {
    users_collection: Collection<UserInfo>,
}

impl LinksRepositoryProvider {
    pub fn new(db: &Database) -> Self {
        let collection_name = std::env::var(LINKS_COLLECTION_NAME_KEY)
            .unwrap_or_else(|_| LINKS_COLLECTION_NAME_DEFAULT.to_owned());
        let links_collection = db.collection::<LinkItem>(&collection_name);
        Self { links_collection }
    }
}

impl UsersRepositoryProvider {
    pub fn new(db: &Database) -> Self {
        let collection_name = std::env::var(USERS_COLLECTION_NAME_KEY)
            .unwrap_or_else(|_| USERS_COLLECTION_NAME_DEFAULT.to_owned());
        let users_collection = db.collection::<UserInfo>(&collection_name);
        Self { users_collection }
    }
}

#[async_trait]
impl LinksRepository for LinksRepositoryProvider {
    async fn find(&self, query: &LinkQuery) -> Result<Vec<LinkItem>> {
        let db_query =
            to_document(query).map_err(|_| AppError::Database("to_document failed".into()))?;
        let result = self
            .links_collection
            .find(db_query, None)
            .await
            .map_err(|e| AppError::Database(format!("find() {e:?}")))?;
        Ok(result.try_collect().await.unwrap_or_else(|_| vec![]))
    }

    async fn get(&self, query: &LinkQuery) -> Result<LinkItem> {
        let db_query =
            to_document(query).map_err(|_| AppError::Database("to_document failed".into()))?;
        let item = self
            .links_collection
            .find_one(db_query, None)
            .await
            .map_err(|e| AppError::Database(format!("find_one() {e:?}")))?;
        item.ok_or_else(|| AppError::LinkNotFound(query.id().to_owned()))
    }

    async fn create(&self, item: &LinkItem) -> Result<LinkItem> {
        let result = self
            .links_collection
            .insert_one(item, None)
            .await
            .map_err(|e| AppError::Database(format!("insert_one() {e:?}")))?;

        let id = result.inserted_id.as_object_id().map_or_else(
            || Err(AppError::Database("unexpected inserted_id()".into())),
            |id| Ok(id.to_hex()),
        )?;
        let query = doc! {"_id": result.inserted_id};
        let update = doc! {"$set": doc! { "id": &id } };
        self.links_collection
            .update_one(query, update, None)
            .await
            .map_err(|e| AppError::Database(format!("update_one() {e:?}")))?;

        Ok(LinkItemBuilder::from(item.clone()).id(&id).build())
    }

    async fn update(&self, query: &LinkQuery, item: &LinkItem) -> Result<LinkItem> {
        let db_query =
            to_document(&query).map_err(|_| AppError::Database("to_document failed".into()))?;
        let opts = ReplaceOptions::builder().upsert(true).build();
        self.links_collection
            .replace_one(db_query, item, Some(opts))
            .await
            .map_err(|e| AppError::Database(format!("replace_one() {e:?}")))?;
        Ok(item.clone())
    }

    async fn delete(&self, query: &LinkQuery) -> Result<()> {
        let db_query =
            to_document(&query).map_err(|_| AppError::Database("to_document failed".into()))?;
        self.links_collection
            .delete_one(db_query, None)
            .await
            .map_err(|e| AppError::Database(format!("delete_one() {e:?}")))?;
        Ok(())
    }
}

#[async_trait]
impl UsersRepository for UsersRepositoryProvider {
    async fn get(&self, query: &UserQuery) -> Result<UserInfo> {
        let db_query =
            to_document(query).map_err(|_| AppError::Database("to_document failed".into()))?;
        let item = self
            .users_collection
            .find_one(db_query, None)
            .await
            .map_err(|e| AppError::Database(format!("find_one() {e:?}")))?;
        item.ok_or_else(|| AppError::UserNotFound(query.email().to_owned()))
    }

    async fn create(&self, info: &UserInfo) -> Result<UserInfo> {
        let result = self
            .users_collection
            .insert_one(info, None)
            .await
            .map_err(|e| AppError::Database(format!("insert_one() {e:?}")))?;

        let id = result.inserted_id.as_object_id().map_or_else(
            || Err(AppError::Database("unexpected inserted_id()".into())),
            |id| Ok(id.to_hex()),
        )?;
        let query = doc! {"_id": result.inserted_id};
        let update = doc! {"$set": doc! { "id": &id } };
        self.users_collection
            .update_one(query, update, None)
            .await
            .map_err(|e| AppError::Database(format!("update_one() {e:?}")))?;

        Ok(UserInfoBuilder::from(info.clone()).id(&id).build())
    }
}
