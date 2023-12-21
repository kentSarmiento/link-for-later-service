use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    email: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Token {
    jwt: String,
}

impl Claims {
    pub fn new(email: &str) -> Self {
        Self {
            email: email.to_string(),
        }
    }
}

impl Token {
    pub fn new(jwt: &str) -> Self {
        Self {
            jwt: jwt.to_string(),
        }
    }

    pub fn jwt(&self) -> &str {
        &self.jwt
    }
}
