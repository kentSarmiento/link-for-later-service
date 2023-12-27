use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Claims {
    sub: String, // email
    iat: usize,  // creation time
    exp: usize,  // expiration time
}

impl Claims {
    pub fn new(sub: &str, iat: usize, exp: usize) -> Self {
        Self {
            sub: sub.to_owned(),
            iat,
            exp,
        }
    }

    pub fn id(&self) -> &str {
        &self.sub
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Token {
    jwt: String,
}

impl Token {
    pub fn new(jwt: &str) -> Self {
        Self {
            jwt: jwt.to_owned(),
        }
    }

    pub fn jwt(&self) -> &str {
        &self.jwt
    }
}
