use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Feed {
    pub id: String,
    pub title: String,
    pub link: String,
    pub description: Option<String>,
    pub categories: Vec<String>,
    pub content: Option<String>,
    pub author: Vec<String>,
    pub source: Source,
    pub images: Option<String>,
    pub published: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Source {
    pub name: String,
    pub url: Vec<String>,
    pub image: Option<String>,
    pub rss: String,
}

impl Feed {
    pub fn description(&self) -> &Option<String> {
        &self.description
    }
}
