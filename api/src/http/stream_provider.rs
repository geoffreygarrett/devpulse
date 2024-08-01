// use std::convert::Infallible;
//
// use async_trait::async_trait;
// use axum::response::sse::{Event, KeepAlive, Sse};
// use futures::Stream;
//
// /// Function to relay or finish any stream as SSE
// pub async fn relay_stream<T: StreamProvider>(
//     provider: T, mode: StreamMode,
// ) -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
//     let mut stream = provider.create_stream(mode).await;
//
//     let relay_stream = async_stream::stream! {
//         while let Some(response) = stream.next().await {
//             match response {
//                 Ok(content) => {
//                     // Split the content by newlines and construct multiple `data:` lines
//                     for line in content.split('\n') {
//                         let event = Event::default().data(line.to_string());
//                         yield Ok(event);
//                     }
//                 }
//                 Err(e) => {
//                     eprintln!("Error while streaming response: {:?}", e);
//                     // Optionally, yield an error event or handle it accordingly
//                 }
//             }
//         }
//     };
//
//     Sse::new(relay_stream).keep_alive(
//         KeepAlive::new()
//             .interval(std::time::Duration::from_secs(10))
//             .text("keep-alive-text"),
//     )
// }
//
// #[derive(Clone, Copy)]
// pub enum StreamMode {
//     Relay,
//     Finish,
// }
//
// #[async_trait]
// pub trait StreamProvider {
//     async fn create_stream(
//         mode: StreamMode,
//     ) -> Box<dyn Stream<Item = Result<String, Box<dyn std::error::Error>>> + Unpin + Send>;
// }
