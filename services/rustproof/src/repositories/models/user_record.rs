use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "adapter-postgres", derive(sqlx::FromRow))]
pub struct UserRecord {
    pub instance_id: Uuid,
    pub id: Uuid,
    pub aud: String,
    pub role: String,
    pub email: String,
    pub encrypted_password: String,
    pub confirmed_at: Option<DateTime<Utc>>,
    pub invited_at: Option<DateTime<Utc>>,
    pub confirmation_token: String,
    pub confirmation_sent_at: Option<DateTime<Utc>>,
    pub recovery_token: String,
    pub recovery_sent_at: Option<DateTime<Utc>>,
    pub email_change_token_current: String,
    pub email_change: Option<String>,
    pub email_change_sent_at: Option<DateTime<Utc>>,
    pub last_sign_in_at: Option<DateTime<Utc>>,
    pub raw_app_meta_data: sqlx::types::Json<HashMap<String, serde_json::Value>>,
    pub raw_user_meta_data: sqlx::types::Json<HashMap<String, serde_json::Value>>,
    pub is_super_admin: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
