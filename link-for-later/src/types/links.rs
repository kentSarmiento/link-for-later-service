use serde::{Deserialize, Serialize};

use super::request::PostLink;

#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct LinkItem {
    id: Option<String>,
    owner: String,
    url: String,
    title: String,
    description: String,
    created_at: String,
    updated_at: String,
}

impl From<PostLink> for LinkItem {
    fn from(post_link: PostLink) -> Self {
        Self {
            owner: post_link.owner,
            url: post_link.url,
            ..Default::default()
        }
    }
}

impl LinkItem {
    pub fn id(&self, id: &str) -> Self {
        Self {
            id: Some(id.to_string()),
            ..self.clone()
        }
    }

    pub fn created_at(&self, created_at: &str) -> Self {
        Self {
            created_at: created_at.to_string(),
            ..self.clone()
        }
    }

    pub fn updated_at(&self, updated_at: &str) -> Self {
        Self {
            updated_at: updated_at.to_string(),
            ..self.clone()
        }
    }
}

#[cfg(test)]
impl LinkItem {
    pub fn new(owner: &str, url: &str) -> Self {
        Self {
            owner: owner.to_string(),
            url: url.to_string(),
            ..Default::default()
        }
    }
}
