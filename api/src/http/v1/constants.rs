pub const TAG_API_DOCS: &str = "API Documentation";
pub const TAG_REPOSITORY_ANALYSIS: &str = "Repository Analysis";
pub const TAG_DEVELOPER_ANALYSIS: &str = "Developer Analysis";

pub const SWAGGER_UI_PATH: &str = "/swagger-ui";
pub const BASE_API_DOCS_PATH: &str = "/api-docs";
pub const OPENAPI_JSON: &str = "/openapi.json";
pub const OPENAPI_YAML: &str = "/openapi.yaml";
pub const RAPIDOC_PATH: &str = "/rapidoc";
pub const ROOT_PATH: &str = "/";
pub const COMMIT_RANGE_PATH: &str = "/repository/commit-range";
pub const DEVELOPER_PERFORMANCE_PATH: &str = "/developers/{username}/performance";


// Use functions to concatenate the strings at runtime
pub fn openapi_json_path() -> String {
    format!("{}{}", BASE_API_DOCS_PATH, OPENAPI_JSON)
}

pub fn openapi_yaml_path() -> String {
    format!("{}{}", BASE_API_DOCS_PATH, OPENAPI_YAML)
}

