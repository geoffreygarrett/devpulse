use http::HeaderValue;
use reqwest::Body;
use std::io::Write;
pub struct MultipartBuilder {
    boundary: String,
    parts: Vec<(String, String)>,
    buffer: Vec<u8>,
}

impl MultipartBuilder {
    pub fn new(boundary: &str) -> Self {
        Self {
            boundary: boundary.to_string(),
            parts: Vec::new(),
            buffer: Vec::new(),
        }
    }

    pub fn add_part(mut self, content_type: &str, content: &str) -> Self {
        self.parts.push((content_type.to_string(), content.to_string()));
        self
    }

    pub fn build(mut self) -> (HeaderValue, Body) {
        use std::fmt::Write;

        for (content_type, content) in self.parts {
            write!(&mut self.buffer, "--{}\r\n", self.boundary).unwrap();
            write!(&mut self.buffer, "Content-Type: {}\r\n\r\n", content_type).unwrap();
            write!(&mut self.buffer, "{}\r\n", content).unwrap();
        }
        write!(&mut self.buffer, "--{}--\r\n", self.boundary).unwrap();

        let content_type = format!("multipart/alternative; boundary={}", self.boundary);
        let header_value = HeaderValue::from_bytes(content_type.as_bytes()).unwrap_or_else(|_| {
            HeaderValue::from_static("multipart/alternative; boundary=fallback_boundary")
        });

        (header_value, Body::from(self.buffer))
    }
}