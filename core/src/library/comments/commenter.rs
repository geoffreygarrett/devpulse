// use async_trait::async_trait;
//
// use crate::comments::comment::Comment;
//
// #[async_trait]
// pub trait Commenter {
//     async fn post_comment(&self, pr_id: &str, comment: Comment) -> Result<()>;
//     async fn post_comments(&self, pr_id: &str, comments: Vec<Comment>) -> Result<()> {
//         for comment in comments {
//             self.post_comment(pr_id, comment).await?;
//         }
//         Ok(())
//     }
//
//     async fn post_inline_comment(&self, pr_id: &str, comment: Comment) -> Result<()>;
//     async fn post_inline_comments(&self, pr_id: &str, comments: Vec<Comment>) -> Result<()> {
//         for comment in comments {
//             self.post_inline_comment(pr_id, comment).await?;
//         }
//         Ok(())
//     }
//
//     async fn post_general_comment(&self, pr_id: &str, message: &str) -> Result<()>;
//     async fn post_general_comments(&self, pr_id: &str, messages: Vec<String>) -> Result<()> {
//         for message in messages {
//             self.post_general_comment(pr_id, &message).await?;
//         }
//         Ok(())
//     }
// }
