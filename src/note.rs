use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};

#[derive(Serialize, Deserialize, Debug)]
pub struct Note {
    pub title: String,
    pub content: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub links: Vec<String>,
    pub tags: Vec<String>,
    pub version: u32,
}

impl Note {
    pub fn new(title: String) -> Self {
        let now = Utc::now();
        Self {
            title,
            content: String::new(),
            created_at: now,
            updated_at: now,
            links: Vec::new(),
            tags: Vec::new(),
            version: 1,
        }
    }

    pub fn update_content(&mut self, content: String) {
        self.content = content;
        self.updated_at = Utc::now();
        self.version += 1;
    }

    pub fn add_link(&mut self, link: String) {
        if !self.links.contains(&link) {
            self.links.push(link);
            self.updated_at = Utc::now();
        }
    }

    pub fn add_tag(&mut self, tag: String) {
        if !self.tags.contains(&tag) {
            self.tags.push(tag);
            self.updated_at = Utc::now();
        }
    }
}