use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct LinkItem {
    id: String,
    owner: String,
    url: String,
    title: String,
    description: String,
    created_at: String,
    updated_at: String,
}
