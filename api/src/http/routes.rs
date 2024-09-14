use axum::{
    Router,
    routing::get,
};
use axum::response::Redirect;
use axum_typed_routing::TypedRouter;
// use crate::http::middleware::RateLimitLayer;
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;
use utoipa::Path;
// use tower::ServiceBuilder;
use utoipa_rapidoc::RapiDoc;
use utoipa_scalar::{Scalar, Servable as ScalarServable};
use utoipa_swagger_ui::SwaggerUi;

use crate::http::*;
use crate::http::api_doc::API_DOC;

pub(crate) fn create_router() -> Router {
    let doc = API_DOC.clone();


    Router::new()
        .route("/", get(|| async { Redirect::permanent("/scalar") }))
        .typed_route(controllers::chat::data_handler)
        .typed_route(controllers::chat_2::chat_relay)
        .typed_route(controllers::openapi::get_openapi_yaml)
        .typed_route(controllers::pull_request::create_pull_request_analysis)
        .typed_route(controllers::developer::get_developer_performance)
        .merge(SwaggerUi::new("/swagger").url("/openapi.json", doc.clone()))
        .merge(Scalar::with_url("/scalar", doc.clone()))
        .typed_route(controllers::operational::health_check)
        .typed_route(controllers::version::version)
        .fallback(controllers::not_found::handler_404)
        .layer(ServiceBuilder::new().layer(TraceLayer::new_for_http()).into_inner())
}

// fn two_serve_dirs() -> Router {
//     // you can also have two `ServeDir`s nested at different paths
//     let serve_dir_from_assets = ServeDir::new("assets");
//     let serve_dir_from_dist = ServeDir::new("dist");
//
//     Router::new()
//         .nest_service("/assets", serve_dir_from_assets)
//         .nest_service("/dist", serve_dir_from_dist)
