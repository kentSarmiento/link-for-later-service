use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct LinkItem {
    pub id: String,
    pub owner: String,
    pub url: String,
    pub title: String,
    pub description: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct UserInfo {
    pub id: String,
    pub email: String,
    pub password: String,
    pub verified: bool,
    pub created_at: String,
    pub updated_at: String,
}
