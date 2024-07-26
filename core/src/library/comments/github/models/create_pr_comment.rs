use derive_builder::Builder;
use serde::{Deserialize, Serialize};

// use crate::library::comments::prelude::*;

/// Parameters for creating a review comment on a pull request.
#[derive(Debug, Serialize, Deserialize, Builder, Default)]
#[builder(setter(into, strip_option), default)]
pub struct CreatePRComment {
    /// The text of the review comment.
    pub body: String,

    /// The SHA of the commit needing a comment.
    pub commit_id: String,

    /// The relative path to the file that necessitates a comment.
    pub path: String,

    /// The line of the blob in the pull request diff that the comment applies to.
    pub line: usize,

    /// The side of the diff that the pull request's changes appear on.
    /// Can be LEFT or RIGHT.
    pub side: String,

    /// The first line in the pull request diff that your multi-line comment applies to.
    #[builder(default)]
    pub start_line: Option<usize>,

    /// The starting side of the diff that the comment applies to.
    /// Can be LEFT or RIGHT.
    #[builder(default)]
    pub start_side: Option<String>,

    /// The ID of the review comment to reply to.
    #[builder(default)]
    pub in_reply_to: Option<u64>,

    /// The level at which the comment is targeted. Can be one of: line, file.
    #[builder(default)]
    pub subject_type: Option<String>,
}

impl CreatePRComment {
    /// Creates a new instance of `CreatePRCommentParams` with mandatory parameters.
    pub fn new(body: String, commit_id: String, path: String, line: usize, side: String) -> Self {
        CreatePRComment {
            body,
            commit_id,
            path,
            line,
            side,
            start_line: None,
            start_side: None,
            in_reply_to: None,
            subject_type: None,
        }
    }
}

// impl From<CreateComment> for CreatePRComment {
//     fn from(comment: Comment) -> Self {
//         CreatePRCommentParams {
//             body: comment.body,
//             commit_id: "commit_sha_placeholder".to_string(), // This should be dynamically assigned
//             path: comment.path.unwrap_or_default(),
//             line: comment.line.unwrap_or(1), // Default to the first line if unspecified
//             side: comment.side.unwrap_or("RIGHT".to_string()),
//             start_line: comment.start_line,
//             start_side: comment.side, // Assuming start_side and side are the same if unspecified
//         }
//     }
// }
