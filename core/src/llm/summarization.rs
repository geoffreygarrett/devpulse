use std::collections::HashMap;

use anyhow::Result;
use futures::future::join_all;
use tera::{Context, Tera};
// # Get details for a specific commit
// curl \
// "https://dev.azure.com/geoffreygarrett/devpulse/_apis/git/repositories/devpulse/commits/9deb6681ce5d28f5b3203b5b052c43121f4686c6?api-version=6.0"
curl "https://dev.azure.com/geoffreygarrett/devpulse/_apis/git/repositories/devpulse/commits/9deb6681ce5d28f5b3203b5b052c43121f4686c6?api-version=6.0" | jq

use crate::BaseClient;
use crate::llms::llm_client::LlmClient;
use crate::prompt::format_prompt;
use crate::settings::Settings;

#[derive(Debug, Clone)]
pub struct SummarizationClient {
    base: BaseClient,
    prompt_file_diff: String,
    prompt_conventional_commit_prefix: String,
    prompt_commit_summary: String,
    prompt_commit_title: String,
    output_conventional_commit: bool,
    output_conventional_commit_prefix_format: String,
    output_show_per_file_summary: bool,
}

impl SummarizationClient {
    pub fn new(settings: Settings, client: Box<dyn LlmClient>) -> Result<Self> {
        let base = BaseClient::new(settings.clone(), client.into());
        let prompt_settings = settings.prompt.unwrap_or_default();
        let output_settings = settings.output.unwrap_or_default();

        Ok(Self {
            base,
            prompt_file_diff: prompt_settings.file_diff.unwrap_or_default(),
            prompt_conventional_commit_prefix: prompt_settings
                .conventional_commit_prefix
                .unwrap_or_default(),
            prompt_commit_summary: prompt_settings.commit_summary.unwrap_or_default(),
            prompt_commit_title: prompt_settings.commit_title.unwrap_or_default(),
            output_conventional_commit: output_settings.conventional_commit.unwrap_or(true),
            output_conventional_commit_prefix_format: output_settings
                .conventional_commit_prefix_format
                .unwrap_or_default(),
            output_show_per_file_summary: output_settings.show_per_file_summary.unwrap_or(false),
        })
    }

    pub async fn get_commit_message(&self, file_diffs: Vec<&str>) -> Result<String> {
        let summaries = self.process_file_diffs(file_diffs).await?;

        let summary_points = summaries
            .iter()
            .map(|(file_name, completion)| format!("[{file_name}]\n{completion}"))
            .collect::<Vec<String>>()
            .join("\n");

        let mut message = String::with_capacity(1024);

        let (title, summary, prefix) = join_all(vec![
            self.commit_title(&summary_points),
            self.commit_summary(&summary_points),
            self.conventional_commit_prefix(&summary_points),
        ])
        .await
        .into_iter()
        .collect::<Result<Vec<_>>>()?;

        message.push_str(&format!("{title}\n\n{summary}\n\n"));

        if self.output_show_per_file_summary {
            for (file_name, completion) in &summaries {
                if !completion.is_empty() {
                    message.push_str(&format!("[{file_name}]\n{completion}\n"));
                }
            }
        }

        let message = self.deduplicate_lines(&message);
        let mut translated_message = self.base.translate_commit(&message).await?;

        if !prefix.is_empty() {
            let formatted_prefix = self.format_prefix(&prefix)?;
            translated_message.insert_str(0, &formatted_prefix);
        }

        Ok(translated_message)
    }

    async fn process_file_diffs(&self, file_diffs: Vec<&str>) -> Result<HashMap<String, String>> {
        let summaries = file_diffs
            .into_iter()
            .map(|file_diff| {
                let file_diff = file_diff.to_owned();
                let cloned_self = self.clone();
                tokio::spawn(async move { cloned_self.process_file_diff(&file_diff).await })
            })
            .collect::<Vec<_>>();

        let results = join_all(summaries).await;

        let mut summary_for_file = HashMap::with_capacity(results.len());
        for res in results {
            if let Ok(Some((k, v))) = res {
                summary_for_file.insert(k, v);
            }
        }

        Ok(summary_for_file)
    }

    async fn process_file_diff(&self, file_diff: &str) -> Option<(String, String)> {
        util::get_file_name_from_diff(file_diff).and_then(|file_name| {
            if self.base.should_ignore(&file_name) {
                warn!("skipping {file_name} due to file_ignore setting");
                None
            } else {
                let completion = self.diff_summary(&file_name, file_diff).await.ok();
                Some((file_name.to_string(), completion.unwrap_or_default()))
            }
        })
    }

    async fn diff_summary(&self, file_name: &str, file_diff: &str) -> Result<String> {
        debug!("summarizing file: {}", file_name);
        let prompt =
            format_prompt(&self.prompt_file_diff, HashMap::from([("file_diff", file_diff)]))?;
        self.base.get_completion(&prompt).await
    }

    async fn conventional_commit_prefix(&self, summary_points: &str) -> Result<String> {
        if !self.output_conventional_commit {
            return Ok(String::new());
        }
        let prompt = format_prompt(
            &self.prompt_conventional_commit_prefix,
            HashMap::from([("summary_points", summary_points)]),
        )?;
        let completion = self.base.get_completion(&prompt).await?;
        Ok(match completion.to_ascii_lowercase().trim() {
            "build" | "chore" | "ci" | "docs" | "feat" | "fix" | "perf" | "refactor" | "style"
            | "test" => completion.to_string(),
            _ => String::new(),
        })
    }

    async fn commit_summary(&self, summary_points: &str) -> Result<String> {
        let prompt = format_prompt(
            &self.prompt_commit_summary,
            HashMap::from([("summary_points", summary_points)]),
        )?;
        self.base.get_completion(&prompt).await
    }

    async fn commit_title(&self, summary_points: &str) -> Result<String> {
        let prompt = format_prompt(
            &self.prompt_commit_title,
            HashMap::from([("summary_points", summary_points)]),
        )?;
        self.base.get_completion(&prompt).await
    }

    fn deduplicate_lines(&self, message: &str) -> String {
        let mut lines = message.lines().collect::<Vec<&str>>();
        lines.dedup();
        lines.join("\n")
    }

    fn format_prefix(&self, prefix: &str) -> Result<String> {
        let mut ctx = Context::new();
        ctx.insert("prefix", prefix);
        Tera::one_off(&self.output_conventional_commit_prefix_format, &ctx, false)
            .map_err(Into::into)
    }
}
