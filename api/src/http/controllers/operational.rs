use std::convert::Infallible;
use std::sync::Arc;
use std::time::Duration;

use axum::{
    Extension,
    extract::Query,
    response::{IntoResponse, Sse},
    response::sse::{Event, KeepAlive},
};
use futures_util::StreamExt;
use serde_json;
use tokio_stream::wrappers::IntervalStream;
use utoipa::ToResponse;

use auto_route::route;

use crate::models::ServerState;

#[derive(utoipa::IntoParams, serde::Deserialize, Default)]
#[into_params(style = Form, parameter_in = Query)]
pub(crate) struct StreamQueryParams {
    #[serde(default)]
    #[serde(rename = "stream")]
    stream: Option<bool>,

    #[serde(default)]
    interval: Option<u64>,
}

/// Health Check
///
/// This endpoint is used to check the health status of the API server.
/// It provides information about the server's current status and uptime.
/// This can be useful for monitoring the server's health and ensuring
/// that it is running as expected.
#[route(
    get,
    path = "/health",
    responses(
        (status = 200, description = "Health check response", body = HealthCheck)
    ),
    params(
        StreamQueryParams,
    ),
    tag = "Operational"
)]
pub(crate) async fn health_check(
    Extension(state): Extension<Arc<tokio::sync::RwLock<ServerState>>>,
    params: Option<Query<StreamQueryParams>>,
) -> impl IntoResponse {
    // Default interval to 5 seconds if not provided
    let default_interval = Duration::from_secs(5);

    // Extract the `stream` and `interval` parameters
    let (should_stream, interval) = match params {
        Some(Query(StreamQueryParams { stream, interval })) => {
            (stream.unwrap_or(false), interval.unwrap_or(5))
        }
        None => (false, 5),
    };

    // if should_stream {
        // let interval_duration = Duration::from_secs(interval);
        // let health_stream =
        //     IntervalStream::new(tokio::time::interval(interval_duration)).then(move |_| {
        //         let s = state.clone();
        //         async move {
        //             let check = s.read().await.health_check().await;
        //             match serde_json::to_string(&check) {
        //                 Ok(data) => Ok::<_, Infallible>(Event::default().data(data)),
        //                 Err(e) => {
        //                     Ok::<_, Infallible>(Event::default().data(e.to_string()).event("ERROR"))
        //                 }
        //             }
        //         }
        //     });

        // Sse::new(health_stream)
        //     .keep_alive(KeepAlive::new().text("keep-alive-text"))
        //     .into_response()
    // } else {
    //     let response = state.read().await.health_check().await;
    //     axum::Json(response).into_response()
    // }
    "Hello, World!"
}
