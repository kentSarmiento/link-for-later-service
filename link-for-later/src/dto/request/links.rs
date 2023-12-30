use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Default, Serialize, Deserialize, Validate)]
pub struct Item {
    #[validate(url)]
    url: String,
    #[serde(default)]
    title: String,
    #[serde(default)]
    description: String,
    #[serde(default)]
    word_count: usize,
    #[serde(default)]
    reading_time: usize,
    #[serde(default)]
    summary: String,
    #[serde(default)]
    label: String,
}

impl Item {
    pub fn url(&self) -> &str {
        &self.url
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn description(&self) -> &str {
        &self.description
    }

    pub const fn word_count(&self) -> usize {
        self.word_count
    }

    pub const fn reading_time(&self) -> usize {
        self.reading_time
    }

    pub fn summary(&self) -> &str {
        &self.summary
    }

    pub fn label(&self) -> &str {
        &self.label
    }

    #[cfg(test)]
    pub fn new(url: &str) -> Self {
        Self {
            url: url.to_owned(),
            ..Default::default()
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Query {
    #[serde(skip_serializing_if = "String::is_empty")]
    id: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    owner: String,
}

impl Query {
    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn owner(&self) -> &str {
        &self.owner
    }
}

#[derive(Default)]
pub struct QueryBuilder {
    id: String,
    owner: String,
}

impl QueryBuilder {
    pub fn new(id: &str, owner: &str) -> Self {
        Self {
            id: id.to_owned(),
            owner: owner.to_owned(),
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

    pub fn build(self) -> Query {
        Query {
            id: self.id,
            owner: self.owner,
        }
    }
}
