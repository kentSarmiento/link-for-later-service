#![allow(dead_code)]
use axum::async_trait;

use crate::entity::{LinkItem, UserInfo};

#[derive(Default)]
pub struct Repository {}

#[async_trait]
impl super::Repository for Repository {
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
