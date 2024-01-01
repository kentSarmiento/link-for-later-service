use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Item {
    id: String,
    owner: String,
    url: String,
    title: String,
    description: String,
    word_count: usize,
    reading_time: usize,
    summary: String,
    label: String,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl Item {
    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn owner(&self) -> &str {
        &self.owner
    }

    pub fn url(&self) -> &str {
        &self.url
    }

    pub const fn created_at(&self) -> &DateTime<Utc> {
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
    word_count: usize,
    reading_time: usize,
    summary: String,
    label: String,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
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

    pub const fn word_count(mut self, word_count: usize) -> Self {
        self.word_count = word_count;
        self
    }

    pub const fn reading_time(mut self, reading_time: usize) -> Self {
        self.reading_time = reading_time;
        self
    }

    pub fn summary(mut self, summary: &str) -> Self {
        self.summary = summary.to_owned();
        self
    }

    pub fn label(mut self, label: &str) -> Self {
        self.label = label.to_owned();
        self
    }

    pub fn created_at(mut self, created_at: &DateTime<Utc>) -> Self {
        self.created_at = created_at.to_owned();
        self
    }

    pub fn updated_at(mut self, updated_at: &DateTime<Utc>) -> Self {
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
            word_count: self.word_count,
            reading_time: self.reading_time,
            summary: self.summary,
            label: self.label,
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
            word_count: item.word_count,
            reading_time: item.reading_time,
            summary: item.summary,
            label: item.label,
            created_at: item.created_at,
            updated_at: item.updated_at,
        }
    }
}
