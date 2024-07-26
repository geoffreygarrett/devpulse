use devpulse_core::comments::prelude::*;

// mod commenter;
#[tokio::main]
async fn main() {
    let comments = vec![
        Comment::builder()
            .file("src/main.rs")
            .line(Some(10))
            .message("Consider refactoring this function to improve readability.")
            .build()
            .unwrap(),
        Comment::builder()
            .file("src/main.rs")
            .line(Some(20))
            .message("This function is too long. Consider breaking it up into smaller functions.")
            .build()
            .unwrap(),
    ];
}
