use derive_builder::Builder;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Builder)]
pub struct Comment {
    pub file: String,
    pub line: Option<usize>,          // Current line number of the comment
    pub end_line: Option<usize>,      // Optional end line number for multiline comments
    pub original_line: Option<usize>, // Original line number of the comment
    pub original_end_line: Option<usize>, // Original end line for multiline comments
    pub message: String,
    pub position: Option<usize>,          // Position in the diff
    pub original_position: Option<usize>, // Original position in the diff
    pub commit_id: String,                // Commit ID where the comment is made
    pub original_commit_id: String, // The original commit ID from the time of the first comment
}
