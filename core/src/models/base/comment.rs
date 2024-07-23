use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Comment {
    id: u32,
    author_id: u32,
    content: String,
    created_at: DateTime<Utc>,
}

impl Comment {
    pub fn new(id: u32, author_id: u32, content: String, created_at: DateTime<Utc>) -> Self {
        Self { id, author_id, content, created_at }
    }
}
