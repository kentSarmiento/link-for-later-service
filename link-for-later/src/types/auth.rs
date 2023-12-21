use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Claims {
    sub: String, // email
    exp: usize,  // expiration time
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Token {
    jwt: String,
}

impl Claims {
    pub fn new(sub: &str, exp: usize) -> Self {
        Self {
            sub: sub.to_string(),
            exp,
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
