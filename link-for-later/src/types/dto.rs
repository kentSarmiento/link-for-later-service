use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Validate)]
pub struct LinkItemRequest {
    #[validate(url)]
    url: String,
    #[serde(default = "String::new")]
    title: String,
    #[serde(default = "String::new")]
    description: String,
}

impl LinkItemRequest {
    pub fn url(&self) -> &str {
        &self.url
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn description(&self) -> &str {
        &self.description
    }
}

pub struct LinkQuery {
    id: String,
    owner: String,
}

impl LinkQuery {
    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn owner(&self) -> &str {
        &self.owner
    }
}

#[derive(Default)]
pub struct LinkQueryBuilder {
    id: String,
    owner: String,
}

impl LinkQueryBuilder {
    pub fn new(id: &str, owner: &str) -> Self {
        Self {
            id: id.to_string(),
            owner: owner.to_string(),
        }
    }

    pub fn owner(mut self, owner: &str) -> Self {
        self.owner = owner.to_string();
        self
    }

    pub fn build(self) -> LinkQuery {
        LinkQuery {
            id: self.id,
            owner: self.owner,
        }
    }
}

#[derive(Serialize, Deserialize, Validate)]
pub struct UserInfoRequest {
    #[validate(email)]
    email: String,
    password: String,
}

impl UserInfoRequest {
    pub fn email(&self) -> &str {
        &self.email
    }

    pub fn password(&self) -> &str {
        &self.password
    }
}

pub struct UserQuery {
    email: String,
}

impl UserQuery {
    pub fn email(&self) -> &str {
        &self.email
    }
}

#[derive(Default)]
pub struct UserQueryBuilder {
    email: String,
}

impl UserQueryBuilder {
    pub fn new(email: &str) -> Self {
        Self {
            email: email.to_string(),
        }
    }

    pub fn build(self) -> UserQuery {
        UserQuery { email: self.email }
    }
}

#[derive(Serialize, Deserialize)]
pub struct AuthResponse {
    token: String,
}

impl AuthResponse {
    pub fn new(token: &str) -> Self {
        Self {
            token: token.to_string(),
        }
    }
}
