#![allow(dead_code)]
use std::str::FromStr;

use axum::async_trait;
use bson::doc;
use mongodb::{options::ClientOptions, Client, Database};
use rand::Rng;

use crate::entity::{LinkItem, UserInfo};

const MONGODB_URI_KEY: &str = "MONGODB_URI";
const MONGODB_DATABASE_NAME_KEY: &str = "MONGODB_DATABASE_NAME";

const LINKS_COLLECTION_NAME_KEY: &str = "LINKS_COLLECTION_NAME";
const USERS_COLLECTION_NAME_KEY: &str = "USERS_COLLECTION_NAME";

pub struct Repository {}

impl Default for Repository {
    fn default() -> Self {
        let mut rng = rand::thread_rng();
        let id = rng.gen::<u32>();

        std::env::set_var(LINKS_COLLECTION_NAME_KEY, format!("v{}/links", id));
        std::env::set_var(USERS_COLLECTION_NAME_KEY, format!("v{}/users", id));
        Self {}
    }
}

#[async_trait]
impl super::Repository for Repository {
    async fn count_links(&self) -> u64 {
        database()
            .await
            .collection::<LinkItem>(&std::env::var(LINKS_COLLECTION_NAME_KEY).unwrap())
            .count_documents(None, None)
            .await
            .unwrap()
    }

    async fn get_link(&self, id: &str) -> LinkItem {
        let db_query = doc! {"_id": bson::oid::ObjectId::from_str(id).unwrap()};
        database()
            .await
            .collection(&std::env::var(LINKS_COLLECTION_NAME_KEY).unwrap())
            .find_one(db_query, None)
            .await
            .unwrap()
            .unwrap()
    }

    async fn add_link(&self, owner: &str, url: &str) -> String {
        let collection = database()
            .await
            .collection(&std::env::var(LINKS_COLLECTION_NAME_KEY).unwrap());

        let document = doc! {"id": "1", "owner": owner, "url": url, "title": "", "description": "", "created_at": "", "updated_at": ""};
        let result = collection.insert_one(document.clone(), None).await.unwrap();

        let id = result.inserted_id.as_object_id().unwrap().to_hex();
        let update = doc! {"$set": doc! { "id": &id } };
        collection.update_one(document, update, None).await.unwrap();

        id
    }

    async fn count_users(&self) -> u64 {
        database()
            .await
            .collection::<UserInfo>(&std::env::var(USERS_COLLECTION_NAME_KEY).unwrap())
            .count_documents(None, None)
            .await
            .unwrap()
    }

    async fn get_user(&self, email: &str) -> UserInfo {
        let db_query = doc! {"email": email};
        database()
            .await
            .collection(&std::env::var(USERS_COLLECTION_NAME_KEY).unwrap())
            .find_one(db_query, None)
            .await
            .unwrap()
            .unwrap()
    }

    async fn add_user(&self, email: &str, password: &str) -> String {
        let collection = database()
            .await
            .collection(&std::env::var(USERS_COLLECTION_NAME_KEY).unwrap());

        let document = doc! {"id": "1", "email": email, "password": password, "verified": true, "created_at": "", "updated_at": ""};
        let result = collection.insert_one(document.clone(), None).await.unwrap();

        let id = result.inserted_id.as_object_id().unwrap().to_hex();
        let update = doc! {"$set": doc! { "id": &id } };
        collection.update_one(document, update, None).await.unwrap();

        id
    }
}

pub async fn database() -> Database {
    let uri = std::env::var(MONGODB_URI_KEY).unwrap();
    let database_name = std::env::var(MONGODB_DATABASE_NAME_KEY).unwrap();

    let client_options = ClientOptions::parse(uri).await.unwrap();
    let client = Client::with_options(client_options).unwrap();
    client.database(&database_name)
}
