use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Default, Serialize, Deserialize, Validate)]
pub struct LinkItemRequest {
    #[validate(url)]
    url: String,
    #[serde(default = "String::default")]
    title: String,
    #[serde(default = "String::default")]
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

    #[cfg(test)]
    pub fn new(url: &str) -> Self {
        Self {
            url: url.to_string(),
            ..Default::default()
        }
    }
}

#[derive(Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct LinkQuery {
    #[serde(skip_serializing_if = "String::is_empty")]
    id: String,
    #[serde(skip_serializing_if = "String::is_empty")]
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

    #[cfg(test)]
    pub fn new(email: &str, password: &str) -> Self {
        Self {
            email: email.to_string(),
            password: password.to_string(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize, PartialEq, Eq)]
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

/*
#[cfg(test)]
pub mod tests {
    use super::LinkItemRequest;

    #[derive(Default)]
    pub struct LinkItemRequestBuilder {
        url: String,
        title: String,
        description: String,
    }

    impl LinkItemRequestBuilder {
        pub fn new(url: &str) -> Self {
            Self {
                url: url.to_string(),
                ..Default::default()
            }
        }

        pub fn title(mut self, title: &str) -> Self {
            self.title = title.to_string();
            self
        }

        pub fn description(mut self, description: &str) -> Self {
            self.description = description.to_string();
            self
        }

        pub fn build(self) -> LinkItemRequest {
            LinkItemRequest {
                url: self.url,
                title: self.title,
                description: self.description,
            }
        }
    }
}
*/
