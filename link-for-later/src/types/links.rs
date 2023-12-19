use chrono::Utc;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::request::{PostLink, PutLink};

#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct LinkItem {
    id: String,
    owner: String,
    url: String,
    title: String,
    description: String,
    created_at: String,
    updated_at: String,
}

impl From<PostLink> for LinkItem {
    fn from(post_link: PostLink) -> Self {
        Self {
            owner: post_link.owner,
            url: post_link.url,
            id: Uuid::new_v4().to_string(),
            created_at: Utc::now().to_rfc3339(),
            ..Default::default()
        }
    }
}

impl From<PutLink> for LinkItem {
    fn from(put_link: PutLink) -> Self {
        Self {
            owner: put_link.owner,
            url: put_link.url,
            title: put_link.title,
            description: put_link.description,
            updated_at: Utc::now().to_rfc3339(),
            ..Default::default()
        }
    }
}

impl LinkItem {
    pub fn merge(&self, other: &Self) -> Self {
        let id = if other.id.is_empty() {
            self.id.clone()
        } else {
            other.id.clone()
        };
        let owner = if other.owner.is_empty() {
            self.owner.clone()
        } else {
            other.owner.clone()
        };
        let url = if other.url.is_empty() {
            self.url.clone()
        } else {
            other.url.clone()
        };
        let title = if other.title.is_empty() {
            self.title.clone()
        } else {
            other.title.clone()
        };
        let description = if other.description.is_empty() {
            self.description.clone()
        } else {
            other.description.clone()
        };
        let created_at = if other.created_at.is_empty() {
            self.created_at.clone()
        } else {
            other.created_at.clone()
        };
        let updated_at = if other.updated_at.is_empty() {
            self.updated_at.clone()
        } else {
            other.updated_at.clone()
        };
        Self {
            id,
            owner,
            url,
            title,
            description,
            created_at,
            updated_at,
        }
    }
}

#[cfg(test)]
impl LinkItem {
    pub fn new(owner: &str, url: &str) -> Self {
        Self {
            owner: owner.to_string(),
            url: url.to_string(),
            ..Default::default()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merge_link_item() {
        let link_item_1 = LinkItem {
            id: "1111".to_string(),
            owner: "1".to_string(),
            url: "http://url".to_string(),
            created_at: "1/Jan/2020:19:03:58 +0000".to_string(),
            ..Default::default()
        };
        let link_item_2 = LinkItem {
            title: "Sample".to_string(),
            description: "Sample url".to_string(),
            updated_at: "1/Jan/2021:19:03:58 +0000".to_string(),
            ..Default::default()
        };
        let expected_link_item = LinkItem {
            id: "1111".to_string(),
            owner: "1".to_string(),
            url: "http://url".to_string(),
            title: "Sample".to_string(),
            description: "Sample url".to_string(),
            created_at: "1/Jan/2020:19:03:58 +0000".to_string(),
            updated_at: "1/Jan/2021:19:03:58 +0000".to_string(),
        };

        let merged_link_item_1 = link_item_1.merge(&link_item_2);
        assert_eq!(merged_link_item_1, expected_link_item);

        let merged_link_item_2 = link_item_2.merge(&link_item_1);
        assert_eq!(merged_link_item_2, expected_link_item);
    }
}
