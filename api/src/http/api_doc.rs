use lazy_static::lazy_static;
use utoipa::{OpenApi, ToSchema};
use utoipa::openapi;
use utoipa::openapi::OpenApiBuilder;

const VERSION: &str = env!("CARGO_PKG_VERSION");
const DESCRIPTION: &str = env!("CARGO_PKG_DESCRIPTION");
const HOMEPAGE: &str = env!("CARGO_PKG_HOMEPAGE");
const REPOSITORY: &str = env!("CARGO_PKG_REPOSITORY");
const PRODUCTION_SERVER: &str = "https://devpulse.shuttleapp.rs";
const PRODUCTION_SERVER_DESCRIPTION: &str = "Production server";
const LOCAL_SERVER: &str = "http://127.0.0.1:8081";
const LOCAL_SERVER_DESCRIPTION: &str = "Local development server";

#[derive(OpenApi, ToSchema)]
#[openapi(
    tags(
        (
            name = "Documentation",
            description = "Endpoints for accessing API documentation in JSON and YAML formats."
        ),
        (
            name = "Repository",
            description = "Endpoints for conducting detailed analyses of repository activities including commit range analysis."
        ),
        (
            name = "Developer",
            description = "Endpoints for retrieving and analyzing developer performance metrics to gauge productivity and contributions."
        ),
        (
            name = "Pull Request",
            description = "Endpoints for analyzing pull request activities to better understand review dynamics and collaboration patterns."
        )
    ),
    paths(
        crate::http::controllers::repository::create_commit_range_analysis,
        crate::http::controllers::developer::get_developer_performance,
        crate::http::controllers::pull_request::create_pull_request_analysis,
        crate::http::controllers::openapi::get_openapi_json,
        crate::http::controllers::openapi::get_openapi_yaml
    ),
    components(
        responses(
            crate::models::TooManyRequests,
            crate::models::BadRequest,
            crate::models::Unauthorized,
            crate::models::InternalServerError,
            crate::models::NotImplemented,
        ),
        schemas(
            crate::models::CommitRangeRequest,
            crate::models::CommitRangeAnalysis,
            crate::models::CommitRangeDetails,
            crate::models::ResponseDetail,
            crate::models::ResponseFormat,
            crate::models::RepositoryContribution,
            crate::models::DeveloperPerformanceAnalysis,
            crate::models::Contributor,
            crate::errors::DevPulseError
        )
    )
)]
pub struct ApiDoc;

lazy_static! {
    /// This is an example for using doc comment attributes
    pub static ref API_DOC: openapi::OpenApi = generate_openapi();
}

pub(crate) fn generate_openapi() -> openapi::OpenApi {
    let builder: OpenApiBuilder = ApiDoc::openapi().into();
    builder
        .info(
            openapi::InfoBuilder::new()
                .title("DevPulse API")
                .version(VERSION)
                .description(Some(DESCRIPTION))
                .terms_of_service(Some(HOMEPAGE))
                .contact(Option::from(
                    openapi::info::ContactBuilder::new()
                        .name(Some("Geoffrey Garrett".to_string()))
                        .url(Some("https://github.com/geoffreygarrett"))
                        .build(),
                ))
                .license(Option::from(
                    openapi::LicenseBuilder::new()
                        .name("MIT".to_string())
                        .url(Some(REPOSITORY))
                        .build(),
                ))
                .build(),
        )
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
        .build()
}
