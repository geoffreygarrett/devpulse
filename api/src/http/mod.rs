pub(crate) use routes::*;

pub(crate) mod api_doc;
pub(crate) mod controllers;
pub(crate) mod middleware;
pub(crate) mod routes;

pub const TAG_DEVELOPER_ANALYSIS: &str = "Developer";

pub const SWAGGER_UI_PATH: &str = "/swagger-ui";
pub const BASE_API_DOCS_PATH: &str = "/api-docs";
pub const OPENAPI_JSON: &str = "/openapi.json";
pub const OPENAPI_YAML: &str = "/openapi.yaml";
pub const RAPIDOC_PATH: &str = "/rapidoc";

// Use functions to concatenate the strings at runtime
pub fn openapi_json_path() -> String {
    format!("{}{}", BASE_API_DOCS_PATH, OPENAPI_JSON)
}

pub fn openapi_yaml_path() -> String {
    format!("{}{}", BASE_API_DOCS_PATH, OPENAPI_YAML)
}
