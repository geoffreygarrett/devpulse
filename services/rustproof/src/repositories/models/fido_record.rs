use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "adapter-postgres", derive(sqlx::FromRow))]

pub struct FidoCredentialsRecord {
    pub id: Uuid,
    pub user_id: Uuid,
    pub credential_id: Vec<u8>,
    pub public_key: Vec<u8>,
    pub sign_count: i64,
    pub aaguid: Option<Uuid>,
    pub transports: Option<Vec<String>>,
    pub created_at: DateTime<Utc>,
    pub last_used_at: Option<DateTime<Utc>>,
    pub display_name: Option<String>,
}
