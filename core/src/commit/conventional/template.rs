fn setup_templates() -> handlebars::Handlebars<'static> {
    let mut handlebars = handlebars::Handlebars::new();
    handlebars.register_template_string("commit_message", r#"
{{emoji}} {{type}}({{scope}}): {{description}}
{{#if body}}
{{body}}
{{/if}}
{{#if footer}}
{{footer}}
{{/if}}
{{#if breaking_change}}
BREAKING CHANGE: {{breaking_change_details}}
{{/if}}
"#).expect("Template should compile without error");
    handlebars
}

/// Renders a commit message using a Handlebars template, now including emojis.
fn render_commit_message(commit: &Commit, handlebars: &handlebars::Handlebars) -> String {
    let data = serde_json::json!({
        "emoji": commit.commit_type.emoji().as_char(),
        "type": format!("{:?}", commit.commit_type),
        "scope": commit.scope,
        "description": commit.description,
        "body": commit.body,
        "footer": commit.footer,
        "breaking_change": commit.breaking_change,
        "breaking_change_details": "Introduces changes that might break backward compatibility.",
    });
    handlebars.render("commit_message", &data).expect("Rendering should succeed")
}
