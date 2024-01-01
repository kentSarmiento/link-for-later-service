use axum::async_trait;

use link_for_later_types::entity::{LinkItem, UserInfo};

use super::Repository;

#[derive(Default)]
pub struct RepositoryProvider {}

#[async_trait]
impl Repository for RepositoryProvider {
    async fn count_links(&self) -> u64 {
        unimplemented!()
    }

    async fn get_link(&self, _id: &str) -> LinkItem {
        unimplemented!()
    }

    async fn add_link(&self, _owner: &str, _url: &str) -> String {
        unimplemented!()
    }

    async fn count_users(&self) -> u64 {
        unimplemented!()
    }

    async fn get_user(&self, _email: &str) -> UserInfo {
        unimplemented!()
    }

    async fn add_user(&self, _email: &str, _password: &str) -> String {
        unimplemented!()
    }
}
