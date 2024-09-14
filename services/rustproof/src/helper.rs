use axum::http::header::ALLOW;
use axum::http::{Method, StatusCode};
use axum::routing::{options, MethodRouter};

/// Builder to create an OPTIONS response with allowed methods.
pub struct OptionsBuilder {
    allowed_methods: Vec<Method>,
}

impl OptionsBuilder {
    pub fn new() -> Self {
        Self {
            allowed_methods: Vec::new(),
        }
    }

    pub fn allow(mut self, method: Method) -> Self {
        self.allowed_methods.push(method);
        self
    }

    /// Build the handler that returns the options response.
    pub fn build(self) -> MethodRouter {
        let methods = self
            .allowed_methods
            .iter()
            .map(Method::to_string)
            .collect::<Vec<_>>()
            .join(", ");

        options(|| async move {
            (
                StatusCode::NO_CONTENT,
                [(ALLOW, methods)],
            )
        })
    }
}
