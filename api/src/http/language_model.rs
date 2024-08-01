use async_trait::async_trait;
use futures::Stream;

#[derive(Clone)]
pub struct Message {
    pub role: String,
    pub content: String,
}

#[async_trait]
pub trait LanguageModelService {
    async fn chat(
        &self, messages: Vec<Message>, stream: bool,
    ) -> Box<dyn Stream<Item = Result<String, Box<dyn std::error::Error>>> + Unpin + Send>;
}
