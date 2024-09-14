use chrono::DateTime;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use sqlx::types::Uuid;
use std::collections::HashMap;

#[derive(Debug)]
#[cfg_attr(feature = "adapter-postgres", derive(sqlx::FromRow))]
pub struct RefreshTokenRecord {
    pub id: Uuid,
    pub user_id: Uuid,
    pub token: String,
    pub parent_token_id: Option<Uuid>,
    pub revoked: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub session_id: Option<Uuid>,
    pub instance_id: Uuid,
}
