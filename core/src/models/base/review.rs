use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::Contributor;

#[derive(Debug, Serialize, Deserialize)]
pub struct Review {
    id: u32,
    reviewer: Contributor,
    status: ReviewStatus,
    submitted_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ReviewStatus {
    Approved,
    ChangesRequested,
    Commented,
}

impl Review {
    pub fn new(id: u32, reviewer: Contributor, status: ReviewStatus, submitted_at: DateTime<Utc>) -> Self {
        Self { id, reviewer, status, submitted_at }
    }
}
