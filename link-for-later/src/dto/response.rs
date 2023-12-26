use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Login {
    token: String,
}

impl Login {
    pub fn new(token: &str) -> Self {
        Self {
            token: token.to_string(),
        }
    }
}
