// annotation_client.rs
use std::collections::HashMap;
use anyhow::Result;
use futures::future::join_all;
use crate::BaseClient;
use crate::settings::Settings;
use crate::llms::llm_client::LlmClient;
use crate::prompt::format_prompt;
use crate::util;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
struct Annotation {
    file_name: String,
    start_line: usize,
    end_line: usize,
    start_column: usize,
    end_column: usize,
    comment: String,
}

#[derive(Debug, Clone)]
pub struct AnnotationClient {
    base: BaseClient,
    prompt_annotation: String,
}

impl AnnotationClient {
    pub fn new(settings: Settings, client: Box<dyn LlmClient>) -> Result<Self> {
        let base = BaseClient::new(settings.clone(), client.into());
        let prompt_settings = settings.prompt.unwrap_or_default();

        Ok(Self {
            base,
            prompt_annotation: prompt_settings.annotation.unwrap_or_default(),
        })
    }

    pub async fn get_annotations(&self, file_diffs: Vec<&str>) -> Result<Vec<Annotation>> {
        let annotations = self.process_file_diffs(file_diffs).await?;
        Ok(annotations)
    }

    async fn process_file_diffs(&self, file_diffs: Vec<&str>) -> Result<Vec<Annotation>> {
        let annotations = file_diffs
            .into_iter()
            .map(|file_diff| {
                let file_diff = file_diff.to_owned();
                let cloned_self = self.clone();
                tokio::spawn(async move { cloned_self.process_file_diff(&file_diff).await })
            })
            .collect::<Vec<_>>();

        let results = join_all(annotations).await;

        let mut all_annotations = Vec::with_capacity(results.len());
        for res in results {
            if let Ok(Some(annotation)) = res {
                all_annotations.push(annotation);
            }
        }

        Ok(all_annotations)
    }

    async fn process_file_diff(&self, file_diff: &str) -> Option<Annotation> {
        util::get_file_name_from_diff(file_diff).and_then(|file_name| {
            if self.base.should_ignore(&file_name) {
                warn!("skipping {file_name} due to file_ignore setting");
                None
            } else {
                let completion = self.annotate_diff(&file_name, file_diff).await.ok();
                completion
            }
        })
    }

    async fn annotate_diff(&self, file_name: &str, file_diff: &str) -> Result<Annotation> {
        debug!("annotating file: {}", file_name);
        let prompt = format_prompt(
            &self.prompt_annotation,
            HashMap::from([("file_diff", file_diff)]),
        )?;
        let completion = self.base.get_completion(&prompt).await?;

        let annotation: Annotation = serde_json::from_str(&completion)?;
        Ok(annotation)
    }
}
