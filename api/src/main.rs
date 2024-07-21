mod http;
mod models;
mod errors;
mod conversion;
mod grpc;

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let router = http::create_router();
    Ok(router.into())
}