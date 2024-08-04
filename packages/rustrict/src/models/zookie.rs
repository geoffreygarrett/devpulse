use std::time::{SystemTime, UNIX_EPOCH};

use crate::zookie::Zookie;

impl Zookie {
    pub fn new() -> Self {
        let value = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs()
            .to_be_bytes()
            .to_vec();
        Self { value }
    }

    pub fn from_bytes(bytes: Vec<u8>) -> Self {
        Self { value: bytes }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        self.value.clone()
    }

    pub fn to_timestamp(&self) -> u64 {
        u64::from_be_bytes(self.value.clone().try_into().unwrap())
    }
}
