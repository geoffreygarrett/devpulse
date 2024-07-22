use std::task::{Context, Poll};

use axum::{
    body::Body,
    http::{header, Request as HttpRequest, StatusCode},
    response::Response,
};
use base64::decode;
use futures_util::future::BoxFuture;
use tower::{Layer, Service};

#[derive(Clone)]
pub struct AuthLayer {
    username: String,
    password: String,
    bearer_token: String,
}

impl AuthLayer {
    pub fn new(username: String, password: String, bearer_token: String) -> Self {
        Self {
            username,
            password,
            bearer_token,
        }
    }
}

impl<S> Layer<S> for AuthLayer {
    type Service = AuthMiddleware<S>;

    fn layer(&self, inner: S) -> Self::Service {
        AuthMiddleware {
            inner,
            username: self.username.clone(),
            password: self.password.clone(),
            bearer_token: self.bearer_token.clone(),
        }
    }
}

#[derive(Clone)]
pub struct AuthMiddleware<S> {
    inner: S,
    username: String,
    password: String,
    bearer_token: String,
}

impl<S> Service<HttpRequest<Body>> for AuthMiddleware<S>
where
    S: Service<HttpRequest<Body>, Response = Response<Body>> + Send + 'static,
    S::Future: Send + 'static,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = BoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, request: HttpRequest<Body>) -> Self::Future {
        let auth_header = request
            .headers()
            .get(header::AUTHORIZATION)
            .and_then(|header| header.to_str().ok());

        let valid = if let Some(auth_header) = auth_header {
            if auth_header.starts_with("Basic ") {
                let encoded_credentials = &auth_header[6..];
                if let Ok(decoded_credentials) = decode(encoded_credentials) {
                    if let Ok(credentials) = String::from_utf8(decoded_credentials) {
                        let mut parts = credentials.splitn(2, ':');
                        if let (Some(username), Some(password)) = (parts.next(), parts.next()) {
                            username == self.username && password == self.password
                        } else {
                            false
                        }
                    } else {
                        false
                    }
                } else {
                    false
                }
            } else if auth_header.starts_with("Bearer ") {
                let token = &auth_header[7..];
                token == self.bearer_token
            } else {
                false
            }
        } else {
            false
        };

        let future = self.inner.call(request);

        Box::pin(async move {
            if valid {
                future.await
            } else {
                let response = Response::builder()
                    .status(StatusCode::UNAUTHORIZED)
                    .header(
                        "WWW-Authenticate",
                        "Basic realm=\"Restricted\", Bearer realm=\"Restricted\"",
                    )
                    .body(Body::from("Unauthorized"))
                    .unwrap();
                Ok(response)
            }
        })
    }
}
