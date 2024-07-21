use axum::response::IntoResponse;

/// Returns a simple greeting message.
///
/// # Responses
/// * `200 OK` - Returns a "Hello, world!" message.
///
/// # Examples
/// * Successful request:
///   ```bash
///   curl -X GET "http://localhost:3000/"
///   ```
#[utoipa::path(
    get,
    path = "/",

    responses(
        (status = 200, description = "Successful response with greeting message", body = String)
    ),
    tag = "General"
)]
pub async fn hello_world() -> impl IntoResponse {
    "Hello, world!"
}
