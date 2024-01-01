use std::str::FromStr;

use axum::async_trait;
use bson::doc;
use mongodb::{options::ClientOptions, Client, Database};
use rand::Rng;

use link_for_later_types::entity::{LinkItem, LinkItemBuilder, UserInfo, UserInfoBuilder};

use super::Repository;

const MONGODB_URI_KEY: &str = "MONGODB_URI";
const MONGODB_DATABASE_NAME_KEY: &str = "MONGODB_DATABASE_NAME";

const LINKS_COLLECTION_NAME_KEY: &str = "LINKS_COLLECTION_NAME";
const USERS_COLLECTION_NAME_KEY: &str = "USERS_COLLECTION_NAME";

#[derive(Default)]
pub struct RepositoryProvider {}

#[async_trait]
impl Repository for RepositoryProvider {
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

        let item = LinkItemBuilder::new(url).id("1").owner(owner).build();
        let result = collection.insert_one(item, None).await.unwrap();

        let id = result.inserted_id.as_object_id().unwrap().to_hex();
        let query = doc! {"_id": result.inserted_id.clone()};
        let update = doc! {"$set": doc! { "id": &id } };
        collection.update_one(query, update, None).await.unwrap();

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

        let info = UserInfoBuilder::new(email, password).id("1").build();
        let result = collection.insert_one(info, None).await.unwrap();

        let id = result.inserted_id.as_object_id().unwrap().to_hex();
        let query = doc! {"_id": result.inserted_id.clone()};
        let update = doc! {"$set": doc! { "id": &id } };
        collection.update_one(query, update, None).await.unwrap();

        id
    }
}

impl RepositoryProvider {
    pub fn setup(&self) {
        let mut rng = rand::thread_rng();
        let id = rng.gen::<u32>();

        std::env::set_var(LINKS_COLLECTION_NAME_KEY, format!("v{}/links", id));
        std::env::set_var(USERS_COLLECTION_NAME_KEY, format!("v{}/users", id));
    }
}

pub async fn database() -> Database {
    let uri = std::env::var(MONGODB_URI_KEY).unwrap();
    let database_name = std::env::var(MONGODB_DATABASE_NAME_KEY).unwrap();

    let client_options = ClientOptions::parse(uri).await.unwrap();
    let client = Client::with_options(client_options).unwrap();
    client.database(&database_name)
}
