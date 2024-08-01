//
// use std::sync::Arc;
// use std::time::SystemTime;
// use tokio::sync::RwLock;
// use tower::ServiceBuilder;
// use tower_http::add_extension::AddExtensionLayer;
// use crate::models::ServerState;
//
// #[tokio::main]
// async fn main() -> shuttle_axum::ShuttleAxum {
//     let start_time = Arc::new(RwLock::new(SystemTime::now()));
//     let server_state = Arc::new(RwLock::new(ServerState::new(start_time)));
//
//     let app_state = AppState {
//         server_state: server_state.clone(),
//     };
//
//     let app = Router::new()
//         .route("/health_check", get(get_health_check))
//         .route("/uptime", get(get_uptime))
//         .route("/metrics", get(get_metrics))
//         .route("/application_status", get(get_application_status))
//         .layer(
//             ServiceBuilder::new()
//                 .layer(TraceLayer::new_for_http())
//                 .layer(AddExtensionLayer::new(server_state)),
//         );
//
//     Ok(app.into())
// }

fn main() {}