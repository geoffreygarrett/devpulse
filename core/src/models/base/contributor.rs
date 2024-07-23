use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Contributor {
    id: u32,
    username: String,
    email: String,
}

impl Contributor {
    pub fn new(id: u32, username: String, email: String) -> Self {
        Self { id, username, email }
    }
}
