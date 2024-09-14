use std::task::{Context, Poll};

use axum::{
    http::Request,
    response::{IntoResponse, Response},
};
use futures_util::future::BoxFuture;
use openfga_rs::CheckRequest;
use openfga_rs::open_fga_service_client::OpenFgaServiceClient;
use route_recognizer::Router as RouteRecognizer;
use tower::{Layer, Service};

#[derive(Clone)]
pub struct AuthorizationLayer;

impl<S> Layer<S> for AuthorizationLayer {
    type Service = AuthorizationMiddleware<S>;

    fn layer(&self, inner: S) -> Self::Service {
        AuthorizationMiddleware { inner }
    }
}

#[derive(Clone)]
pub struct AuthorizationMiddleware<S> {
    inner: S,
}
impl<S, B> Service<Request<B>> for AuthorizationMiddleware<S>
where
    S: Service<Request<B>, Response=Response> + Send + 'static,
    S::Future: Send + 'static,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = BoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, request: Request<B>) -> Self::Future {
        let path = request.uri().path().to_string();
        let future = self.inner.call(request);
        let mut router = RouteRecognizer::new();
        router.add("/swagger", ());
        router.add("/swagger/*file", ());
        router.add("/rapidoc", ());

        Box::pin(async move {
            let client = OpenFgaServiceClient::connect("http://localhost:80").await;


                 match client {
                Ok(mut client) => {
                    let check_request = CheckRequest {
                        store_id: "01J4KJ9A19MVRP5SRFAQ90G3J8".to_string(),
                        tuple_key: Some(openfga_rs::CheckRequestTupleKey {
                            user: "user:geoffrey".to_string(),
                            relation: "reader".to_string(),
                            object: format!("endpoint:{}", path),
                        }),
                        contextual_tuples: None,
                        authorization_model_id: "01J4M8HXHREZWPFA7SCRJ2C3QV".to_string(),
                        trace: true,
                        context: None,
                    };

                    match client.check(check_request).await {
                        Ok(response) => {
                            if response.into_inner().allowed {
                                future.await
                            } else {
                                Ok((axum::http::StatusCode::FORBIDDEN, "Forbidden").into_response())
                            }
                        }
                        Err(e) => {
                            println!("Error: {:?}", e);
                            Ok((axum::http::StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error").into_response())
                        }
                    }
                }
                Err(e) => {
                    println!("Connection Error: {:?}", e);
                    Ok((axum::http::StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error").into_response())
                }
            }
        })
    }
}
