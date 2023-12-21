use serde::{Deserialize, Serialize};

use super::{LoginRequest, PostLinkRequest, RegisterRequest};

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

#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct UserInfo {
    pub id: Option<String>,
    pub email: String,
    pub password: String,
    pub verified: bool,
    pub created_at: String,
    pub updated_at: String,
}

impl From<PostLinkRequest> for LinkItem {
    fn from(post_link: PostLinkRequest) -> Self {
        Self {
            owner: post_link.owner,
            url: post_link.url,
            ..Default::default()
        }
    }
}

impl From<RegisterRequest> for UserInfo {
    fn from(registration: RegisterRequest) -> Self {
        Self {
            email: registration.email,
            password: registration.password,
            ..Default::default()
        }
    }
}

impl From<LoginRequest> for UserInfo {
    fn from(login: LoginRequest) -> Self {
        Self {
            email: login.email,
            password: login.password,
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

impl UserInfo {
    pub fn id(&self, id: &str) -> Self {
        Self {
            id: Some(id.to_string()),
            ..self.clone()
        }
    }

    pub fn verified(&self, verified: bool) -> Self {
        Self {
            verified,
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
