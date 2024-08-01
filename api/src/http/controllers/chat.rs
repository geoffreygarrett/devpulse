use std::time::Duration;

use axum::{
    extract::Query,
    response::sse::{Event, KeepAlive, Sse},
};
use axum_core::response::IntoResponse;
use axum_extra::TypedHeader;
use futures::stream::StreamExt;
use headers::UserAgent;
use rand::Rng;
use serde::Deserialize;
use serde::Serialize;
use tokio::time::interval;
use tokio_stream::wrappers::IntervalStream;
use tracing_subscriber;

use auto_route::route;

#[derive(Debug, Clone, Serialize)]
struct MyTestStructure {
    some_test_field: String,
    event_type: String,
}

#[derive(Deserialize, utoipa::IntoParams, Clone, Debug)]
#[into_params(style = Form, parameter_in = Query)]
struct StreamQuery {
    stream: Option<bool>,
}

/// Handler for combined HTTP and SSE response.
#[route(
    get,
    path = "/data",
    operation_id = "data_handler",
    params(
        StreamQuery
    ),
    responses(
        (status = 200, description = "Stream or regular HTTP response of messages"),
    ),
)]
pub async fn data_handler(
    Query(params): Query<StreamQuery>, TypedHeader(user_agent): TypedHeader<UserAgent>,
) -> impl IntoResponse {
    println!("`{}` connected", user_agent.as_str());

    if params.stream.unwrap_or(false) {
        let interval = interval(Duration::from_secs(1));
        let stream = IntervalStream::new(interval)
            .map(|_| {
                let mut rng = rand::thread_rng();
                let event_type = if rng.gen_bool(0.5) { "type1" } else { "type2" };

                let data = if event_type == "type1" {
                    serde_json::to_string(&MyTestStructure {
                        some_test_field: "test_type1".to_string(),
                        event_type: event_type.to_string(),
                    })
                    .unwrap()
                } else {
                    serde_json::to_string(&MyTestStructure {
                        some_test_field: "test_type2".to_string(),
                        event_type: event_type.to_string(),
                    })
                    .unwrap()
                };

                Event::default().data(data).event(event_type)
            })
            .map(|x| Ok::<_, axum::Error>(x));

        Sse::new(stream)
            .keep_alive(
                KeepAlive::new()
                    .interval(Duration::from_secs(10))
                    .text("keep-alive-text"),
            )
            .into_response()
    } else {
        let data = MyTestStructure {
            some_test_field: "test".to_string(),
            event_type: "regular".to_string(),
        };
        axum::Json(data).into_response()
    }
}
