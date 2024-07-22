use crate::http::*;
use tower_http::trace::TraceLayer;

use axum::{
    Router, routing::{post, get},
};
use axum::response::Redirect;
use tower::ServiceBuilder;
use utoipa_rapidoc::RapiDoc;
use utoipa_swagger_ui::SwaggerUi;
use utoipa_redoc::{Redoc, Servable};
use utoipa_scalar::{Scalar, Servable as ScalarServable};
use crate::http::api_doc::API_DOC;

pub(crate) fn create_router() -> Router {
    let doc = API_DOC.clone();
    Router::new()
        .layer(
            ServiceBuilder::new()
                .layer(TraceLayer::new_for_http())
                .layer(middleware::AuthLayer::new("admin".to_string(), "password".to_string(), "token".to_string()))
                .into_inner(),
        )
        .nest(BASE_API_DOCS_PATH, Router::new()
            .route(OPENAPI_YAML, get(controllers::docs::get_openapi_yaml)),
        )
        .merge(SwaggerUi::new(SWAGGER_UI_PATH).url(openapi_json_path(), doc.clone()))
        .merge(RapiDoc::new(openapi_json_path()).path(RAPIDOC_PATH))
        .merge(Redoc::with_url("/redoc", doc.clone()))
        .merge(Scalar::with_url("/scalar", doc.clone()))
        .route(ROOT_PATH, get(|| async { Redirect::permanent(SWAGGER_UI_PATH) }))
        .route(COMMIT_RANGE_PATH, post(controllers::repository::create_commit_range_analysis))
        .route(DEVELOPER_PERFORMANCE_PATH, get(controllers::developer::get_developer_performance))
        .fallback(controllers::not_found::handler_404)
}