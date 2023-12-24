#![allow(dead_code)]
use std::str::FromStr;

use bson::{doc, oid::ObjectId, Bson};
use mongodb::{options::ClientOptions, Client, Database};
use rand::Rng;

use crate::entity::{LinkItem, UserInfo};

pub fn setup() {
    let mut rng = rand::thread_rng();
    let id = rng.gen::<u32>();

    let links_collection = format!("{}/links", id);
    std::env::set_var("LINKS_COLLECTION_NAME", links_collection);

    let users_collection = format!("{}/users", id);
    std::env::set_var("USERS_COLLECTION_NAME", users_collection);
}

pub async fn database() -> Database {
    let uri = std::env::var("MONGODB_URI").unwrap();
    let database_name = std::env::var("MONGODB_DATABASE_NAME").unwrap();

    let client_options = ClientOptions::parse(uri).await.unwrap();
    let client = Client::with_options(client_options).unwrap();
    client.database(&database_name)
}

pub async fn count_links() -> u64 {
    let collection = std::env::var("LINKS_COLLECTION_NAME").unwrap();
    let collection = database().await.collection::<LinkItem>(&collection);

    collection.count_documents(None, None).await.unwrap()
}

pub async fn get_link(id: &str) -> LinkItem {
    let collection = std::env::var("LINKS_COLLECTION_NAME").unwrap();
    let collection = database().await.collection(&collection);

    let id = if let Ok(id) = bson::oid::ObjectId::from_str(id) {
        id
    } else {
        ObjectId::default()
    };
    let db_query = doc! {"_id": id};
    collection.find_one(db_query, None).await.unwrap().unwrap()
}

pub async fn add_link(owner: &str, url: &str) -> String {
    let collection = std::env::var("LINKS_COLLECTION_NAME").unwrap();
    let collection = database().await.collection(&collection);

    let document = doc! {"id": "1", "owner": owner, "url": url, "title": "", "description": "", "created_at": "", "updated_at": ""};
    let result = collection.insert_one(document, None).await.unwrap();

    let id = if let Bson::ObjectId(id) = result.inserted_id {
        id.to_hex()
    } else {
        String::new()
    };
    let query = doc! {"_id": result.inserted_id.clone()};
    let update = doc! {"$set": doc! { "id": &id } };
    collection.update_one(query, update, None).await.unwrap();

    id
}

pub async fn count_users() -> u64 {
    let collection = std::env::var("USERS_COLLECTION_NAME").unwrap();
    let collection = database().await.collection::<UserInfo>(&collection);

    collection.count_documents(None, None).await.unwrap()
}

pub async fn get_user(email: &str) -> UserInfo {
    let collection = std::env::var("USERS_COLLECTION_NAME").unwrap();
    let collection = database().await.collection(&collection);

    let db_query = doc! {"email": email};
    collection.find_one(db_query, None).await.unwrap().unwrap()
}

pub async fn add_user(email: &str, password: &str) -> String {
    let collection = std::env::var("USERS_COLLECTION_NAME").unwrap();
    let collection = database().await.collection(&collection);

    let document = doc! {"id": "1", "email": email, "password": password, "verified": true, "created_at": "", "updated_at": ""};
    let result = collection.insert_one(document, None).await.unwrap();

    let id = if let Bson::ObjectId(id) = result.inserted_id {
        id.to_hex()
    } else {
        String::new()
    };
    let query = doc! {"_id": result.inserted_id.clone()};
    let update = doc! {"$set": doc! { "id": &id } };
    collection.update_one(query, update, None).await.unwrap();

    id
}
