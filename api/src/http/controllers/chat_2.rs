use std::convert::Infallible;
use std::env::var;
use std::time::Duration;

use async_stream::stream;
use axum::extract::Query;
use axum::response::sse::{Event, KeepAlive, Sse};
use axum_core::response::IntoResponse;
use axum_extra::TypedHeader;
use dotenv::dotenv;
use futures::StreamExt;
use headers::UserAgent;
use rs_openai::{
    chat::{ChatCompletionMessageRequestBuilder, CreateChatRequestBuilder, Role},
    OpenAI,
};
use rs_openai::chat::CreateChatRequest;
use serde::Deserialize;
use tracing_subscriber;
use auto_route::route;

#[derive(Deserialize)]
struct StreamQuery {
    stream: Option<bool>,
}

/// Function to initialize the OpenAI client
fn initialize_openai_client() -> OpenAI {
    dotenv().ok();
    let api_key = var("OPENAI_API_KEY").expect("OPENAI_API_KEY not set");

    OpenAI::new(&OpenAI {
        api_key,
        org_id: None,
    })
}

/// Function to create a request for OpenAI chat completion with streaming enabled
fn create_openai_request() -> CreateChatRequest {
    CreateChatRequestBuilder::default()
        .model("gpt-3.5-turbo")
        .messages(vec![ChatCompletionMessageRequestBuilder::default()
            .role(Role::User)
            .content("To Solve LeetCode's problem 81 in Rust.")
            .build()
            .expect("Failed to build chat completion message")])
        .stream(true)
        .build()
        .expect("Failed to build chat request")
}


#[route(
    get,
    path = "/chat-relay",
    operation_id = "chat-relay",
    responses(
        (status = 200, description = "Stream or regular HTTP response of messages"),
    ),
)]
pub async fn chat_relay(
    Query(params): Query<StreamQuery>, TypedHeader(user_agent): TypedHeader<UserAgent>,
) -> impl IntoResponse {
    println!("`{}` connected", user_agent.as_str());

    if params.stream.unwrap_or(false) {
        relay_openai_stream().await.into_response()
    } else {
        "Hello, World!".into_response()
    }
}

/// Function to relay the OpenAI stream as SSE
pub async fn relay_openai_stream() -> Sse<impl futures::Stream<Item = Result<Event, Infallible>>> {
    let client = initialize_openai_client();
    let req = create_openai_request();

    let mut stream = client
        .chat()
        .create_with_stream(&req)
        .await
        .expect("Failed to create OpenAI stream");

    let relay_stream = stream! {
        while let Some(response) = stream.next().await {
            match response {
                Ok(res) => {
                    for choice in res.choices {
                        if let Some(ref content) = choice.delta.content {
                            // Split the content by newlines and construct multiple `data:` lines
                            for line in content.split('\n') {
                                let event = Event::default().data(line.to_string());
                                yield Ok(event);
                            }
                        }
                    }
                }
                Err(e) => {
                    eprintln!("Error while streaming OpenAI response: {:?}", e);
                    // Optionally, yield an error event or handle it accordingly
                }
            }
        }
    };

    Sse::new(relay_stream).keep_alive(
        KeepAlive::new()
            .interval(Duration::from_secs(10))
            .text("keep-alive-text"),
    )
}
