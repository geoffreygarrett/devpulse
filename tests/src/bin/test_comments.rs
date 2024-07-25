use clap::{App, Arg};
use commenter::{Comment, Commenter, GitHubCommenter};
use tokio;

mod commenter;
#[tokio::main]
async fn main() {
    let matches = App::new("Rust Comment CLI")
        .version("1.0")
        .author("Your Name")
        .about("Tests commenting on PRs")
        .arg(
            Arg::with_name("message")
                .short('m')
                .long("message")
                .value_name("MESSAGE")
                .help("Sets the comment message")
                .takes_value(true),
        )
        .get_matches();

    let message = matches
        .value_of("message")
        .unwrap_or("Default comment message!");

    let comment = Comment {
        file: Some("src/main.rs".to_string()),
        line: Some(10),
        message: message.to_string(),
    };

    let commenter = GitHubCommenter {
        api_token: "your_api_token".to_string(),
        base_url: "https://api.github.com".to_string(),
    };

    if let Err(e) = commenter.post_comment("pr_id_here", comment).await {
        eprintln!("Error posting comment: {}", e);
    }
}
