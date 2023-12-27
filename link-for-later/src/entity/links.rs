use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Item {
    id: String,
    owner: String,
    url: String,
    title: String,
    description: String,
    created_at: String,
    updated_at: String,
}

impl Item {
    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn owner(&self) -> &str {
        &self.owner
    }

    #[cfg(test)]
    pub fn url(&self) -> &str {
        &self.url
    }

    pub fn created_at(&self) -> &str {
        &self.created_at
    }
}

#[derive(Default)]
pub struct ItemBuilder {
    id: String,
    owner: String,
    url: String,
    title: String,
    description: String,
    created_at: String,
    updated_at: String,
}

impl ItemBuilder {
    pub fn new(url: &str) -> Self {
        Self {
            url: url.to_owned(),
            ..Default::default()
        }
    }

    pub fn id(mut self, id: &str) -> Self {
        self.id = id.to_owned();
        self
    }

    pub fn owner(mut self, owner: &str) -> Self {
        self.owner = owner.to_owned();
        self
    }

    pub fn url(mut self, url: &str) -> Self {
        self.url = url.to_owned();
        self
    }

    pub fn title(mut self, title: &str) -> Self {
        self.title = title.to_owned();
        self
    }

    pub fn description(mut self, description: &str) -> Self {
        self.description = description.to_owned();
        self
    }

    pub fn created_at(mut self, created_at: &str) -> Self {
        self.created_at = created_at.to_owned();
        self
    }

    pub fn updated_at(mut self, updated_at: &str) -> Self {
        self.updated_at = updated_at.to_owned();
        self
    }

    pub fn build(self) -> Item {
        Item {
            id: self.id,
            owner: self.owner,
            url: self.url,
            title: self.title,
            description: self.description,
            created_at: self.created_at,
            updated_at: self.updated_at,
        }
    }
}

impl From<Item> for ItemBuilder {
    fn from(item: Item) -> Self {
        Self {
            id: item.id,
            owner: item.owner,
            url: item.url,
            title: item.title,
            description: item.description,
            created_at: item.created_at,
            updated_at: item.updated_at,
        }
    }
}
