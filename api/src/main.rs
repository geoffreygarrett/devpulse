#[allow(unused_imports)]

mod conversion;
mod errors;
mod grpc;
mod http;
mod models;

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let router = http::create_router();
    Ok(router.into())
}
