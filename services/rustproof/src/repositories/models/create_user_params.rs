use chrono::{DateTime, Utc};
use uuid::Uuid;

pub struct CreateUserParams {
    pub id: Option<Uuid>,
    pub email: String,
    pub encrypted_password: String,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
    pub confirmation_sent_at: Option<DateTime<Utc>>,

}

impl CreateUserParams {
    pub fn with_email_and_password(email: &str, encrypted_password: &str) -> Self {
        let now = Utc::now();
        Self {
            id: Some(Uuid::new_v4()),
            email: email.to_string(),
            encrypted_password: encrypted_password.to_string(),
            created_at: Some(now),
            updated_at: Some(now),
            confirmation_sent_at: Some(now),
        }
    }
}

