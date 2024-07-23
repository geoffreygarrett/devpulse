use std::sync::Arc;
use std::time::Duration;

use axum::{
    Router,
    routing::{get, post},
};
use axum::body::Body;
use axum::error_handling::HandleErrorLayer;
use axum::http::{Request, StatusCode};
use axum::response::{IntoResponse, Redirect};
// use crate::http::middleware::RateLimitLayer;
use tower::{BoxError, buffer::BufferLayer, limit::RateLimitLayer, ServiceBuilder};
use tower_governor::{governor::GovernorConfigBuilder, GovernorError, GovernorLayer};
use tower_governor::key_extractor::{PeerIpKeyExtractor, SmartIpKeyExtractor};
use tower_http::trace::TraceLayer;
// use tower::ServiceBuilder;
use utoipa_rapidoc::RapiDoc;
use utoipa_redoc::{Redoc, Servable};
use utoipa_scalar::{Scalar, Servable as ScalarServable};
use utoipa_swagger_ui::SwaggerUi;

use crate::http::*;
use crate::http::api_doc::API_DOC;
use crate::models::TooManyRequests;

pub(crate) fn create_router() -> Router {
    // Configure tracing if desired
    // construct a subscriber that prints formatted traces to stdout
    // let subscriber = tracing_subscriber::FmtSubscriber::new();
    // use that subscriber to process traces emitted after this point
    // tracing::subscriber::set_global_default(subscriber).unwrap();

    // Allow bursts with up to five requests per IP address
    // and replenishes one element every two seconds
    // We Box it because Axum 0.6 requires all Layers to be Clone
    // and thus we need a static reference to it
    let governor_conf = Arc::new(
        GovernorConfigBuilder::default()
            .per_second(1)
            .burst_size(5)
            .use_headers()
            .key_extractor(SmartIpKeyExtractor)
            .error_handler(|error| {
                tracing::error!("Rate limiting error: {}", error);
                match error {
                    GovernorError::TooManyRequests { wait_time, .. } => {
                        TooManyRequests::new(Some(wait_time as i32)).into_response()
                    }
                    _ => TooManyRequests::new(None).into_response(),
                }
            })
            .finish()
            .unwrap(),
    );

    let governor_limiter = governor_conf.limiter().clone();
    let interval = Duration::from_secs(60);
    // a separate background task to clean up
    std::thread::spawn(move || loop {
        std::thread::sleep(interval);
        tracing::info!("rate limiting storage size: {}", governor_limiter.len());
        governor_limiter.retain_recent();
    });

    let doc = API_DOC.clone();
    let router = Router::new()
        .nest(
            BASE_API_DOCS_PATH,
            Router::new().route(OPENAPI_YAML, get(controllers::openapi::get_openapi_yaml)),
        )
        .merge(SwaggerUi::new(SWAGGER_UI_PATH).url(openapi_json_path(), doc.clone()))
        .merge(RapiDoc::new(openapi_json_path()).path(RAPIDOC_PATH))
        .merge(Redoc::with_url("/redoc", doc.clone()))
        .merge(Scalar::with_url("/scalar", doc.clone()))
        .route(ROOT_PATH, get(|| async { Redirect::permanent(SWAGGER_UI_PATH) }))
        .route("/pull-request", post(controllers::pull_request::create_pull_request_analysis))
        .route(
            DEVELOPER_PERFORMANCE_PATH,
            get(controllers::developer::get_developer_performance),
        )
        .route(COMMIT_RANGE_PATH, post(controllers::repository::create_commit_range_analysis))
        .layer(ServiceBuilder::new().layer(TraceLayer::new_for_http()))
        .fallback(controllers::not_found::handler_404);

    if (std::env::var("SHUTTLE").is_ok()) {
        router.layer(GovernorLayer {
            config: governor_conf,
        })
    } else {
        router
    }
}
