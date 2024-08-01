use std::env::var;
use std::io::{stdout, Write};

use dotenv::dotenv;
use futures::StreamExt;
// use external_openai as openai;
// // use dotenvy::dotenv;
use rs_openai::{
    chat::{ChatCompletionMessageRequestBuilder, CreateChatRequestBuilder, Role},
    OpenAI,
};

//
// #[tokio::test]
// async fn test_websocket_communication() {
//
//
// }
//

#[tokio::test]
async fn test_me() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let api_key = var("OPENAI_API_KEY").unwrap();

    let client = OpenAI::new(&OpenAI {
        api_key,
        org_id: None,
    });

    // stream mode
    let req = CreateChatRequestBuilder::default()
        .model("gpt-3.5-turbo")
        .messages(vec![ChatCompletionMessageRequestBuilder::default()
            .role(Role::User)
            .content("To Solve LeetCode's problem 81 in Rust.")
            .build()?])
        .stream(true)
        .build()?;

    let mut stream = client.chat().create_with_stream(&req).await?;

    let mut lock = stdout().lock();
    while let Some(response) = stream.next().await {
        response.unwrap().choices.iter().for_each(|choice| {
            if let Some(ref content) = choice.delta.content {
                write!(lock, "{}", content).unwrap();
            }
        });

        stdout().flush()?;
    }

    Ok(())
}
