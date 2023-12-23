use axum::async_trait;

use crate::types::{
    dto::{LinkQuery, UserQuery},
    entity::{LinkItem, UserInfo},
    AppError, Result,
};

use super::{Links, Users};

#[derive(Default)]
pub struct Bare {}

#[async_trait]
impl Links for Bare {
    async fn search(&self, _query: &LinkQuery) -> Result<Vec<LinkItem>> {
        Err(AppError::NotSupported)
    }

    async fn get(&self, _query: &LinkQuery) -> Result<LinkItem> {
        Err(AppError::NotSupported)
    }

    async fn create(&self, _item: &LinkItem) -> Result<LinkItem> {
        Err(AppError::NotSupported)
    }

    async fn update(&self, _id: &str, _item: &LinkItem) -> Result<LinkItem> {
        Err(AppError::NotSupported)
    }

    async fn delete(&self, _item: &LinkItem) -> Result<()> {
        Err(AppError::NotSupported)
    }
}

#[async_trait]
impl Users for Bare {
    async fn search(&self, _query: &UserQuery) -> Result<UserInfo> {
        Err(AppError::NotSupported)
    }

    async fn create(&self, _info: &UserInfo) -> Result<UserInfo> {
        Err(AppError::NotSupported)
    }
}
