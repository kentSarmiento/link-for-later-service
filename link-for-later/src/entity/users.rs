use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Info {
    id: String,
    email: String,
    password: String,
    verified: bool,
    created_at: String,
    updated_at: String,
}

impl Info {
    pub fn email(&self) -> &str {
        &self.email
    }

    pub fn password(&self) -> &str {
        &self.password
    }
}

#[derive(Default)]
pub struct InfoBuilder {
    id: String,
    email: String,
    password: String,
    verified: bool,
    created_at: String,
    updated_at: String,
}

impl InfoBuilder {
    pub fn new(email: &str, password: &str) -> Self {
        Self {
            email: email.to_owned(),
            password: password.to_owned(),
            ..Default::default()
        }
    }

    pub fn id(mut self, id: &str) -> Self {
        self.id = id.to_owned();
        self
    }

    #[allow(clippy::missing_const_for_fn)]
    pub fn verified(mut self, verified: bool) -> Self {
        self.verified = verified;
        self
    }

    pub fn created_at(mut self, created_at: &str) -> Self {
        self.created_at = created_at.to_owned();
        self
    }

    pub fn updated_at(mut self, updated_at: &str) -> Self {
        self.updated_at = updated_at.to_owned();
        self
    }

    pub fn build(self) -> Info {
        Info {
            id: self.id,
            email: self.email,
            password: self.password,
            verified: self.verified,
            created_at: self.created_at,
            updated_at: self.updated_at,
        }
    }
}

impl From<Info> for InfoBuilder {
    fn from(info: Info) -> Self {
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
