use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct LinkItem {
    id: String,
    owner: String,
    url: String,
    title: String,
    description: String,
    created_at: String,
    updated_at: String,
}

impl LinkItem {
    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn owner(&self) -> &str {
        &self.owner
    }

    pub fn created_at(&self) -> &str {
        &self.created_at
    }
}

#[derive(Default)]
pub struct LinkItemBuilder {
    id: String,
    owner: String,
    url: String,
    title: String,
    description: String,
    created_at: String,
    updated_at: String,
}

impl LinkItemBuilder {
    pub fn new(url: &str) -> Self {
        Self {
            url: url.to_string(),
            ..Default::default()
        }
    }

    pub fn id(mut self, id: &str) -> Self {
        self.id = id.to_string();
        self
    }

    pub fn owner(mut self, owner: &str) -> Self {
        self.owner = owner.to_string();
        self
    }

    pub fn url(mut self, url: &str) -> Self {
        self.url = url.to_string();
        self
    }

    pub fn title(mut self, title: &str) -> Self {
        self.title = title.to_string();
        self
    }

    pub fn description(mut self, description: &str) -> Self {
        self.description = description.to_string();
        self
    }

    pub fn created_at(mut self, created_at: &str) -> Self {
        self.created_at = created_at.to_string();
        self
    }

    pub fn updated_at(mut self, updated_at: &str) -> Self {
        self.updated_at = updated_at.to_string();
        self
    }

    pub fn build(self) -> LinkItem {
        LinkItem {
            id: self.id,
            owner: self.owner,
            url: self.url,
            title: self.title,
            description: self.description,
            created_at: self.created_at,
            updated_at: self.updated_at,
        }
    }
}

impl From<LinkItem> for LinkItemBuilder {
    fn from(item: LinkItem) -> Self {
        Self {
            id: item.id,
            owner: item.owner,
            url: item.url,
            title: item.title,
            description: item.description,
            created_at: item.created_at,
            updated_at: item.updated_at,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct UserInfo {
    id: String,
    email: String,
    password: String,
    verified: bool,
    created_at: String,
    updated_at: String,
}

impl UserInfo {
    pub fn email(&self) -> &str {
        &self.email
    }

    pub fn password(&self) -> &str {
        &self.password
    }
}

#[derive(Default)]
pub struct UserInfoBuilder {
    id: String,
    email: String,
    password: String,
    verified: bool,
    created_at: String,
    updated_at: String,
}

impl UserInfoBuilder {
    pub fn new(email: &str, password: &str) -> Self {
        Self {
            email: email.to_string(),
            password: password.to_string(),
            ..Default::default()
        }
    }

    pub fn id(mut self, id: &str) -> Self {
        self.id = id.to_string();
        self
    }

    #[allow(clippy::missing_const_for_fn)]
    pub fn verified(mut self, verified: bool) -> Self {
        self.verified = verified;
        self
    }

    pub fn created_at(mut self, created_at: &str) -> Self {
        self.created_at = created_at.to_string();
        self
    }

    pub fn updated_at(mut self, updated_at: &str) -> Self {
        self.updated_at = updated_at.to_string();
        self
    }

    pub fn build(self) -> UserInfo {
        UserInfo {
            id: self.id,
            email: self.email,
            password: self.password,
            verified: self.verified,
            created_at: self.created_at,
            updated_at: self.updated_at,
        }
    }
}

impl From<UserInfo> for UserInfoBuilder {
    fn from(info: UserInfo) -> Self {
        Self {
            id: info.id,
            email: info.email,
            password: info.password,
            verified: info.verified,
            created_at: info.created_at,
            updated_at: info.updated_at,
        }
    }
}
