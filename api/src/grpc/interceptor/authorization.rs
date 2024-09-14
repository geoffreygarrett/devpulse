use async_trait::async_trait;
use openfga_rs::{CheckRequest, CheckRequestTupleKey};
use openfga_rs::open_fga_service_client::OpenFgaServiceClient;
use tonic::body::BoxBody;
use tonic::codegen::http::Request;
use tonic::Status;
use tonic_middleware::RequestInterceptor;

#[derive(Clone)]
pub struct AuthorizeInterceptor {
    pub fga_client: OpenFgaServiceClient<tonic::transport::Channel>,
}

struct Claims {
    sub: String,

}
#[async_trait]
impl RequestInterceptor for AuthorizeInterceptor {
    async fn intercept(&self, mut req: Request<BoxBody>) -> Result<Request<BoxBody>, Status> {
        let mut client = OpenFgaServiceClient::connect("http://openfga:50051").await.unwrap();
        // Extract claims from request
        let claims = req
            .extensions()
            .get::<Claims>()
            .ok_or_else(|| Status::unauthenticated("Missing claims"))?;

        // Extract the user from claims
        let user = &claims.sub;

        // Use the request path to determine the object
        let path = req.uri().path().to_string();
        let object = format!("resource:{}", path);

        // Set relation based on your logic
        let relation = "reader";

        // Create a check request
        let check_request = CheckRequest {
            store_id: "01J4KJ9A19MVRP5SRFAQ90G3J8".to_string(),
            tuple_key: Some(CheckRequestTupleKey {
                user: user.to_string(),
                relation: relation.to_string(),
                object: object.to_string(),
            }),
            contextual_tuples: None,
            authorization_model_id: "01J4KJ9RCX89JKKB3ZCG3H0X7D".to_string(),
            trace: false,
            context: None,
        };

        // Perform the authorization check
        let allowed = client.check(check_request).await
            .map_err(|_| Status::internal("Authorization check failed")).unwrap().into_inner().allowed;

        // Return request if authorized, otherwise return error
        if allowed {
            Ok(req)
        } else {
            Err(Status::permission_denied("Not authorized"))
        }
    }
}
