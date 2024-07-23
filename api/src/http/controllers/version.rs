// #[derive(Serialize, ToSchema, ToResponse)]
// #[response(status = 200, description = "Application version")]
// pub struct VersionResponse {
//     version: String,
// }
//
// #[utoipa::path(
//     get,
//     path = "/version",
//     responses(
//         (status = 200, response = VersionResponse),
//     ),
//     tag = "General"
// )]
// pub async fn version() -> impl IntoResponse {
//     let version = VersionResponse {
//         version: "1.0.0".to_string(),
//     };
//     version.into_response()
// }
