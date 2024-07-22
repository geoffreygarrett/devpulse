use axum::Router;
use crate::http::v1::controllers;

mod http;

use http::*;


fn fallthrough() -> String {
    "nothing to see here".to_string()
}


#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let router = Router::new()
        .nest("/vasdasdasdasdasd1", v1::router())
        .fallback(fallthrough());

    Ok(router.into())
}