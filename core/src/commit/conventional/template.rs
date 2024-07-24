use handlebars;

use crate::commit::conventional::Commit;

pub(crate) fn setup_templates() -> handlebars::Handlebars<'static> {
    let mut handlebars = handlebars::Handlebars::new();
    handlebars
        .register_template_string(
            "commit_message",
            r#"{{type}}{{#if scope}}({{scope}}){{/if}}: {{description}}{{#if emoji}} {{emoji}}{{/if}}{{#if breaking_change}}
BREAKING CHANGE: {{breaking_change}}{{/if}}{{#if body}}
{{body}}{{/if}}{{#if footer}}
{{footer}}{{/if}}"#,
        )
        .expect("Template should compile without error");
    handlebars
}

/// Adjusted rendering to conditionally include emoji
pub fn render_commit_message(
    commit: &Commit, handlebars: &handlebars::Handlebars,
) -> String {
    let data = serde_json::json!({
        "emoji": if commit.use_emoji { Some(commit.commit_type.emoji().as_char()) } else { None },
        "type": commit.commit_type.as_str(),
        "scope": commit.scope,
        "description": commit.description,
        "body": commit.body,
        "footer": commit.footer,
        "breaking_change": commit.breaking_change,
    });
    handlebars
        .render("commit_message", &data)
        .expect("Rendering should succeed")
}
