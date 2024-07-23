// use serde::{Serialize, Deserialize};
// use super::Contributor;
// use chrono::{DateTime, Utc};
//
// #[derive(Debug, Serialize, Deserialize)]
// pub struct Commit {
//     hash: String,
//     author: Contributor,
//     message: String,
//     timestamp: DateTime<Utc>,
// }
//
// impl Commit {
//     pub fn new(hash: String, author: Contributor, message: String, timestamp: DateTime<Utc>) -> Self {
//         Self { hash, author, message, timestamp }
//     }
// }
