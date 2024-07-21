use crate::http::*;
use tower_http::trace::TraceLayer;

use axum::{
    Router, routing::{post, get},
};
use axum::response::Redirect;
use tonic::IntoRequest;
use tower::ServiceBuilder;
use utoipa_rapidoc::RapiDoc;
use utoipa_swagger_ui::SwaggerUi;
use utoipa_redoc::{Redoc, Servable};
use utoipa_scalar::{Scalar, Servable as ScalarServable};

use utoipa::{OpenApi, ToSchema};
use utoipa::openapi;
use utoipa::openapi::OpenApiBuilder;

const VERSION: &str = env!("CARGO_PKG_VERSION");
const DESCRIPTION: &str = env!("CARGO_PKG_DESCRIPTION");
const HOMEPAGE: &str = env!("CARGO_PKG_HOMEPAGE");
const REPOSITORY: &str = env!("CARGO_PKG_REPOSITORY");
const PRODUCTION_SERVER: &str = "https://devpulse.shuttleapp.rs";
const PRODUCTION_SERVER_DESCRIPTION: &str = "Production server";
const LOCAL_SERVER: &str = "http://localhost:8000";
const LOCAL_SERVER_DESCRIPTION: &str = "Local development server";


#[derive(OpenApi, ToSchema)]
#[openapi(
    paths(
        crate::http::controllers::repository::create_commit_range_analysis,
        crate::http::controllers::developer::get_developer_performance,
        crate::http::controllers::docs::get_openapi_json,
        crate::http::controllers::docs::get_openapi_yaml
    ),
    components(
        schemas(
            crate::models::CommitRangeRequest,
            crate::models::CommitRangeResponse,
            crate::models::CommitRangeDetails,
            crate::models::RepositoryContribution,
            crate::models::DeveloperPerformance,
            crate::models::Contributor,
            crate::errors::DevPulseError,
        )
    )
)]
pub struct ApiDoc;


pub(crate) fn create_router() -> Router {
    let builder: OpenApiBuilder = ApiDoc::openapi().into();
    let doc = builder.info(openapi::InfoBuilder::new()
        .title("DevPulse API")
        .version(VERSION)
        .description(Some(DESCRIPTION))
        .terms_of_service(Some(HOMEPAGE))
        .contact(Option::from(openapi::info::ContactBuilder::new()
            .name(Some("Geoffrey Garrett".to_string()))
            .url(Some("https://github.com/geoffreygarrett"))
            .build()))
        .license(Option::from(openapi::LicenseBuilder::new()
            .name("MIT".to_string())
            .url(Some(REPOSITORY))
            .build()))
        .build())
        .servers(Some(vec![
            openapi::ServerBuilder::new()
                .url(PRODUCTION_SERVER.to_string())
                .description(Some(PRODUCTION_SERVER_DESCRIPTION.to_string()))
                .build(),
            openapi::ServerBuilder::new()
                .url(LOCAL_SERVER.to_string())
                .description(Some(LOCAL_SERVER_DESCRIPTION.to_string()))
                .build(),
        ]))
        .build();

    // controllers::__path_hello_world.into_request().into_parts(
    Router::new()
        .layer(
            ServiceBuilder::new()
                .layer(TraceLayer::new_for_http())
                .layer(middleware::AuthLayer::new("admin".to_string(), "password".to_string(), "token".to_string()))
                .into_inner(),
        )
        .nest(BASE_API_DOCS_PATH, Router::new()
            // .route(OPENAPI_JSON, get(controllers::docs::get_openapi_json))
            .route(OPENAPI_YAML, get(controllers::docs::get_openapi_yaml))
        )
        .merge(SwaggerUi::new(SWAGGER_UI_PATH).url(openapi_json_path(), doc.clone()))
        .merge(RapiDoc::new(openapi_json_path()).path(RAPIDOC_PATH))
        .merge(Redoc::with_url("/redoc", ApiDoc::openapi()))
        .merge(Scalar::with_url("/scalar", ApiDoc::openapi()))
        .route(ROOT_PATH, get(|| async { Redirect::permanent(SWAGGER_UI_PATH) }))
        .route(COMMIT_RANGE_PATH, post(controllers::repository::create_commit_range_analysis))
        .route(DEVELOPER_PERFORMANCE_PATH, get(controllers::developer::get_developer_performance))
}