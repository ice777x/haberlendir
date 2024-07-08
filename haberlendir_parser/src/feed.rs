use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Feed {
    pub id: String,
    pub title: String,
    pub link: String,
    pub description: Option<String>,
    pub content: Option<String>,
    pub author: String,
    pub images: Option<String>,
    pub published: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

impl Feed {
    pub fn description(&self) -> &Option<String> {
        &self.description
    }
}
