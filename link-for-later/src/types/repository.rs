use std::sync::Arc;

use axum::async_trait;
#[cfg(test)]
use mockall::{automock, predicate::*};

use super::{links::LinkItem, AppError, Result};

pub type DynLinks = Arc<dyn Links + Send + Sync>;

#[allow(clippy::used_underscore_binding)]
#[cfg_attr(test, automock)]
#[async_trait]
pub trait Links {
    async fn list(&self) -> Result<Vec<LinkItem>> {
        Err(AppError::NotSupported)
    }

    async fn post(&self, _item: &LinkItem) -> Result<LinkItem> {
        Err(AppError::NotSupported)
    }

    async fn get(&self, _id: &str) -> Result<LinkItem> {
        Err(AppError::NotSupported)
    }

    async fn put(&self, _id: &str) -> Result<LinkItem> {
        Err(AppError::NotSupported)
    }

    async fn delete(&self, _id: &str) -> Result<()> {
        Err(AppError::NotSupported)
    }
}
