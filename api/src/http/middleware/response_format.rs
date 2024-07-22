// use axum::{
//     response::{IntoResponse, Response},
//     async_trait,
//     extract::{FromRequest, RequestParts, TypedHeader},
//     header::{ContentType, HeaderMapExt},
//     http::StatusCode,
// };
// use serde::Serialize;
// use tower_http::response::ResponseExt;
//
// struct ResponseFormatMiddleware<B> {
//     inner: B,
// }
//
// impl<B> ResponseFormatMiddleware<B> {
//     fn new(inner: B) -> Self {
//         ResponseFormatMiddleware { inner }
//     }
// }
//
// #[async_trait]
// impl<B, S> tower_service::Service<axum::http::Request<B>> for ResponseFormatMiddleware<S>
// where
//     B: Send + 'static,
//     S: tower_service::Service<axum::http::Request<B>, Response = Response> + Send + 'static,
//     S::Future: Send,
//     S::Error: Into<axum::BoxError>,
// {
//     type Response = S::Response;
//     type Error = S::Error;
//     type Future = futures_util::future::BoxFuture<'static, Result<Self::Response, Self::Error>>;
//
//     async fn poll_ready(&mut self, cx: &mut std::task::Context<'_>) -> std::task::Poll<Result<(), Self::Error>> {
//         self.inner.poll_ready(cx)
//     }
//
//     fn call(&mut self, req: axum::http::Request<B>) -> Self::Future {
//         let inner = self.inner.clone();
//         let fut = async move {
//             let res = inner.call(req).await?;
//             let res = format_response(res)?;
//             Ok(res)
//         };
//         Box::pin(fut)
//     }
// }
