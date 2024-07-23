// base_client.rs
use std::collections::HashMap;
use std::sync::Arc;

use anyhow::Result;

use crate::llms::llm_client::LlmClient;
use crate::prompt::format_prompt;
use crate::settings::Language;

#[derive(Debug, Clone)]
pub struct BaseClient {
    pub client: Arc<dyn LlmClient>,
    pub file_ignore: Vec<String>,
    pub prompt_translation: String,
    pub output_lang: Language,
}

impl BaseClient {
    pub fn new(settings: Settings, client: Arc<dyn LlmClient>) -> Self {
        let prompt_settings = settings.prompt.unwrap_or_default();
        let output_settings = settings.output.unwrap_or_default();
        Self {
            client,
            file_ignore: settings.file_ignore.unwrap_or_default(),
            prompt_translation: prompt_settings.translation.unwrap_or_default(),
            output_lang: Language::from_str(&output_settings.lang.unwrap_or_default())
                .unwrap_or_default(),
        }
    }

    pub async fn translate_commit(&self, commit_message: &str) -> Result<String> {
        if self.output_lang == Language::En {
            return Ok(commit_message.to_string());
        }
        let prompt = format_prompt(
            &self.prompt_translation,
            HashMap::from([
                ("commit_message", commit_message),
                ("output_language", &self.output_lang.to_string()),
            ]),
        )?;
        self.client.completions(&prompt).await
    }

    pub fn should_ignore(&self, file_name: &str) -> bool {
        self.file_ignore
            .iter()
            .any(|ignore| file_name.contains(ignore))
    }

    pub async fn get_completion(&self, prompt: &str) -> Result<String> {
        self.client.completions(prompt).await
    }
}
