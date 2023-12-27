use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Validate)]
pub struct Info {
    #[validate(email)]
    email: String,
    password: String,
}

impl Info {
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

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Query {
    email: String,
}

impl Query {
    pub fn email(&self) -> &str {
        &self.email
    }
}

#[derive(Default)]
pub struct QueryBuilder {
    email: String,
}

impl QueryBuilder {
    pub fn new(email: &str) -> Self {
        Self {
            email: email.to_owned(),
        }
    }

    pub fn build(self) -> Query {
        Query { email: self.email }
    }
}
