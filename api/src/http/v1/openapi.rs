use utoipa::{OpenApi, ToSchema};
use utoipa::openapi;
use utoipa::openapi::{OpenApiBuilder};


const VERSION: &str = env!("CARGO_PKG_VERSION");
const DESCRIPTION: &str = env!("CARGO_PKG_DESCRIPTION");
const HOMEPAGE: &str = env!("CARGO_PKG_HOMEPAGE");
const REPOSITORY: &str = env!("CARGO_PKG_REPOSITORY");
const PRODUCTION_SERVER: &str = "https://devpulse.shuttleapp.rs/v1";
const PRODUCTION_SERVER_DESCRIPTION: &str = "Production server";
const LOCAL_SERVER: &str = "http://localhost:8000/v1";
const LOCAL_SERVER_DESCRIPTION: &str = "Local development server";


#[derive(OpenApi, ToSchema)]
#[openapi(
    paths(
        super::controllers::repository::create_commit_range_analysis,
        super::controllers::developer::get_developer_performance,
        super::controllers::docs::get_openapi_json,
        super::controllers::docs::get_openapi_yaml
    ),
    components(
        responses(
            super::models::TooManyRequests
        ),
        schemas(
            super::models::TooManyRequests,
            super::models::CommitRangeRequest,
            super::models::CommitRangeResponse,
            super::models::CommitRangeDetails,
            super::models::ResponseDetail,
            super::models::ResponseFormat,
            super::models::RepositoryContribution,
            super::models::DeveloperPerformance,
            super::models::Contributor,
            super::errors::DevPulseError
        )
    )
)]
pub struct ApiDoc;

use lazy_static::lazy_static;

lazy_static! {
    /// This is an example for using doc comment attributes
    pub static ref API_DOC: openapi::OpenApi = generate_openapi();
}

pub(crate) fn generate_openapi() -> openapi::OpenApi {
    let builder: OpenApiBuilder = ApiDoc::openapi().into();
    builder.info(openapi::InfoBuilder::new()
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
        .build()
}



