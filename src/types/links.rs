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
