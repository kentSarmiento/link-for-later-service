use axum::async_trait;

use crate::types::{
    entity::{LinkItem, UserInfo},
    AppError, Result,
};

use super::{Links, Users};

#[derive(Default)]
pub struct Bare {}

#[async_trait]
impl Links for Bare {
    async fn list(&self) -> Result<Vec<LinkItem>> {
        Err(AppError::NotSupported)
    }

    async fn post(&self, _item: &LinkItem) -> Result<LinkItem> {
        Err(AppError::NotSupported)
    }

    async fn get(&self, _id: &str) -> Result<LinkItem> {
        Err(AppError::NotSupported)
    }

    async fn put(&self, _id: &str, _item: &LinkItem) -> Result<LinkItem> {
        Err(AppError::NotSupported)
    }

    async fn delete(&self, _id: &str) -> Result<()> {
        Err(AppError::NotSupported)
    }
}

#[async_trait]
impl Users for Bare {
    async fn add(&self, _info: &UserInfo) -> Result<UserInfo> {
        Err(AppError::NotSupported)
    }

    async fn find_by_user(&self, _user: &str) -> Result<UserInfo> {
        Err(AppError::NotSupported)
    }
}
