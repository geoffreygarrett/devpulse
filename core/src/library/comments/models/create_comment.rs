use derive_builder::Builder;
use serde::{Deserialize, Serialize};

/// A platform-agnostic structure for representing a comment on a code review or pull request.
#[derive(Debug, Serialize, Deserialize, Builder, Default)]
#[builder(
    setter(into, strip_option),
    default,
    build_fn(private, name = "private_build")
)]
pub struct CreateComment {
    /// The main text of the comment.
    pub body: String,

    /// The relative file path within the repository to which the comment applies.
    #[builder(default)]
    pub path: Option<String>,

    /// The specific line in the file to which the comment applies.
    #[builder(default)]
    pub line: Option<usize>,

    /// For multi-line comments, the starting line of the comment.
    #[builder(default)]
    pub start_line: Option<usize>,

    /// For multi-line comments, the end line of the comment.
    #[builder(default)]
    pub end_line: Option<usize>,

    /// Optional: Specifies the side of the diff to which the comment applies.
    /// Common values might include "LEFT", "RIGHT", or None if not applicable.
    #[builder(default)]
    pub side: Option<String>,

    /// Optional: Contextual information about the comment, such as being a reply to another comment.
    #[builder(default)]
    pub in_reply_to: Option<u64>,
}

impl CreateComment {
    /// Creates a new Comment with the essential parameters.
    pub fn builder(body: String) -> CreateCommentBuilder {
        CreateCommentBuilder::default()
    }

    fn validate(&self) -> Result<(), String> {
        if let (Some(start), Some(end)) = (self.start_line, self.end_line) {
            if start > end {
                return Err("start_line must be less than or equal to end_line".into());
            }
        }
        Ok(())
    }
}
