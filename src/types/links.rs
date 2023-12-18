use chrono::Utc;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

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

impl<T: ToString> From<T> for LinkItem {
    fn from(link_url: T) -> Self {
        Self {
            url: link_url.to_string(),
            id: Uuid::new_v4().to_string(),
            created_at: Utc::now().to_rfc3339(),
            ..Default::default()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_link_item_conversions_from_str() {
        let url = "http://link";

        let link_item = LinkItem::from(url);
        assert!(!link_item.id.is_empty());
        assert!(link_item.owner.is_empty());
        assert!(link_item.url == url);
        assert!(link_item.title.is_empty());
        assert!(link_item.description.is_empty());
        assert!(!link_item.created_at.is_empty());
        assert!(link_item.updated_at.is_empty());

        let link_item: LinkItem = url.into();
        assert!(!link_item.id.is_empty());
        assert!(link_item.owner.is_empty());
        assert!(link_item.url == url);
        assert!(link_item.title.is_empty());
        assert!(link_item.description.is_empty());
        assert!(!link_item.created_at.is_empty());
        assert!(link_item.updated_at.is_empty());
    }

    #[test]
    fn test_link_item_conversions_from_string() {
        let url = String::from("http://link");
        let link_item = LinkItem::from(url.clone());
        assert!(!link_item.id.is_empty());
        assert!(link_item.owner.is_empty());
        assert!(link_item.url == url);
        assert!(link_item.title.is_empty());
        assert!(link_item.description.is_empty());
        assert!(!link_item.created_at.is_empty());
        assert!(link_item.updated_at.is_empty());

        let link_item: LinkItem = url.clone().into();
        assert!(!link_item.id.is_empty());
        assert!(link_item.owner.is_empty());
        assert!(link_item.url == url);
        assert!(link_item.title.is_empty());
        assert!(link_item.description.is_empty());
        assert!(!link_item.created_at.is_empty());
        assert!(link_item.updated_at.is_empty());
    }
}
