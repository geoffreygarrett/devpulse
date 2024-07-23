use std::sync::Arc;
use std::time::SystemTime;

use tokio::sync::RwLock;
use tower::ServiceBuilder;
use tower_http::add_extension::AddExtensionLayer;
use tower_http::trace::TraceLayer;

use crate::models::ServerState;

#[allow(unused_imports)]
mod conversion;
mod errors;
mod grpc;
mod http;
mod models;
mod utils;

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let start_time = Arc::new(RwLock::new(SystemTime::now()));
    let server_state = ServerState::new(start_time);

    let router = http::create_router().layer(
        ServiceBuilder::new()
            .layer(TraceLayer::new_for_http())
            .layer(AddExtensionLayer::new(server_state)),
    );

    Ok(router.into())
}
