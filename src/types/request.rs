use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct PostLink {
    url: String,
}

impl ToString for PostLink {
    fn to_string(&self) -> String {
        self.url.clone()
    }
}
