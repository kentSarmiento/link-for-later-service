use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Claims {
    sub: String, // email
    admin: bool, // admin role
    iat: usize,  // creation time
    exp: usize,  // expiration time
}

impl Claims {
    pub fn new(sub: &str, admin: bool, iat: usize, exp: usize) -> Self {
        Self {
            sub: sub.to_owned(),
            admin,
            iat,
            exp,
        }
    }

    pub fn id(&self) -> &str {
        &self.sub
    }

    pub const fn is_admin(&self) -> bool {
        self.admin
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
