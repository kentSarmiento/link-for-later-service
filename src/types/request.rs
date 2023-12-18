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

#[cfg(test)]
impl PostLink {
    pub fn new(url: &str) -> Self {
        Self {
            url: url.to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_post_link_to_string() {
        let link = PostLink::new("http://link");
        assert_eq!(link.to_string(), String::from("http://link"));
    }
}
