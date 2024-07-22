// use super::{Commit, Review, Contributor, Comment};
// use serde::{Serialize, Deserialize};
// use chrono::{DateTime, Utc};
//
// #[derive(Debug, Serialize, Deserialize)]
// pub struct PullRequest {
//     id: u32,
//     title: String,
//     description: String,
//     author: Contributor,
//     commits: Vec<Commit>,
//     reviews: Vec<Review>,
//     comments: Vec<Comment>,
//     created_at: DateTime<Utc>,
//     updated_at: DateTime<Utc>,
// }
//
// impl PullRequest {
//     pub fn new(id: u32, title: String, description: String, author: Contributor, created_at: DateTime<Utc>, updated_at: DateTime<Utc>) -> Self {
//         Self { id, title, description, author, commits: vec![], reviews: vec![], comments: vec![], created_at, updated_at }
//     }
//
//     pub fn add_commit(&mut self, commit: Commit) {
//         self.commits.push(commit);
//     }
//
//     pub fn add_review(&mut self, review: Review) {
//         self.reviews.push(review);
//     }
//
//     pub fn add_comment(&mut self, comment: Comment) {
//         self.comments.push(comment);
//     }
// }
