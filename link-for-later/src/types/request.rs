use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct PostLink {
    pub owner: String,
    pub url: String,
}

#[cfg(test)]
impl PostLink {
    pub fn new(owner: &str, url: &str) -> Self {
        Self {
            owner: owner.to_string(),
            url: url.to_string(),
        }
    }
}
