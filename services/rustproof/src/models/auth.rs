#[derive(Debug)]
pub struct User {
    pub id: uuid::Uuid,
    pub email: String,
}

#[derive(Debug)]
pub struct Session {
    pub access_token: String,
    pub refresh_token: String,
    pub token_type: String,
    pub expires_in: i64,
    pub user_id: uuid::Uuid,
    pub expires_at: chrono::DateTime<chrono::Utc>,
}
