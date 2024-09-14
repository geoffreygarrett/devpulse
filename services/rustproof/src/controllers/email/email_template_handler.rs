use crate::config::TemplateType;
use axum::extract::Path;
use axum::response::{IntoResponse, Response};
use http::header::CONTENT_TYPE;
use http::HeaderMap;
use snafu::ResultExt;
use crate::utils::email::MultipartBuilder;

pub async fn email_template_handler(
    Path((locale, template_type)): Path<(String, TemplateType)>,
    headers: HeaderMap,
) -> impl IntoResponse {
    let cache_control = headers.get("Cache-Control").map(|v| v.to_str().unwrap_or("")).unwrap_or("");

    // Example response for plain text and HTML formats
    let plain_text = format!("{:?} Template in {}", template_type, locale);
    let html_content = format!("<html><body><h1>{:?} Template in {}</h1></body></html>", template_type, locale);

    // Assuming this is within an async function that returns Result<T, anyhow::Error>
    let (content_type, body) = MultipartBuilder::new("boundary42")
        .add_part("text/plain; charset=utf-8", &plain_text)
        .add_part("text/html; charset=utf-8", &html_content)
        .build();

    // Build the final response
    let mut response = Response::builder()
        .header(CONTENT_TYPE, content_type)
        .body(body)
        .unwrap();

    // Add Cache-Control header if present
    if !cache_control.is_empty() {
        response.headers_mut().insert("Cache-Control", cache_control.parse().unwrap());
    }

    response
}