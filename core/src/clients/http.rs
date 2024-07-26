// use std::sync::Arc;
// use std::time::Duration;
//
// use failsafe::{Config, FailurePolicy, Instrument, StateMachine};
// use failsafe::backoff::EqualJittered;
// use failsafe::failure_policy::{ConsecutiveFailures, OrElse, SuccessRateOverTimeWindow};
// use failsafe::futures::CircuitBreaker;
// use nject::injectable;
// use reqwest::{Client, RequestBuilder, Response};
// use serde::Serialize;
// use thiserror::Error;
// use tracing::{error, info, warn};
//
// use crate::clients::models::arc_client::ArcClient;
//
//
//
// #[injectable]
// struct FailsafeClient<POLICY: FailurePolicy + Send + Sync, INSTRUMENT: Instrument + Send + Sync> {
//     client: ArcClient,
//     breaker: ClientBreaker<POLICY, INSTRUMENT>,
// }
//
// impl Default
//     for FailsafeClient<
//         OrElse<SuccessRateOverTimeWindow<EqualJittered>, ConsecutiveFailures<EqualJittered>>,
//         (),
//     >
// {
//     fn default() -> Self {
//         let client = ArcClient::default();
//         let config = Config::new();
//         FailsafeClient::new(client, config)
//     }
// }
//
// impl<POLICY: FailurePolicy + Send + Sync, INSTRUMENT: Instrument + Send + Sync>
//     FailsafeClient<POLICY, INSTRUMENT>
// {
//     fn new(client: ArcClient, config: Config<POLICY, INSTRUMENT>) -> Self {
//         FailsafeClient {
//             client,
//             breaker: ClientBreaker {
//                 circuit_breaker: config.build(),
//             },
//         }
//     }
//
//     async fn execute_request_with_circuit_breaker<F>(
//         &self, func: F,
//     ) -> Result<Response, HttpClientError>
//     where
//         F: FnOnce() -> std::pin::Pin<
//             Box<dyn std::future::Future<Output = Result<Response, HttpClientError>> + Send>,
//         >,
//     {
//         let result = self.breaker.circuit_breaker.call(func()).await;
//         match result {
//             Ok(response) => Ok(response),
//             Err(_) => Err(HttpClientError::CircuitBreakerOpen),
//         }
//     }
//
//     pub fn get(&self, url: &str) -> HttpRequestBuilder<POLICY, INSTRUMENT> {
//         HttpRequestBuilder::new(self, self.client.inner().get(url))
//     }
//
//     pub fn post<T: Serialize>(
//         &self, url: &str, body: &T,
//     ) -> HttpRequestBuilder<POLICY, INSTRUMENT> {
//         HttpRequestBuilder::new(self, self.client.inner().post(url).json(body))
//     }
//
//     pub fn put<T: Serialize>(&self, url: &str, body: &T) -> HttpRequestBuilder<POLICY, INSTRUMENT> {
//         HttpRequestBuilder::new(self, self.client.inner().put(url).json(body))
//     }
//
//     pub fn delete(&self, url: &str) -> HttpRequestBuilder<POLICY, INSTRUMENT> {
//         HttpRequestBuilder::new(self, self.client.inner().delete(url))
//     }
//
//     pub fn head(&self, url: &str) -> HttpRequestBuilder<POLICY, INSTRUMENT> {
//         HttpRequestBuilder::new(self, self.client.inner().head(url))
//     }
//
//     pub fn patch<T: Serialize>(
//         &self, url: &str, body: &T,
//     ) -> HttpRequestBuilder<POLICY, INSTRUMENT> {
//         HttpRequestBuilder::new(self, self.client.inner().patch(url).json(body))
//     }
//
//     pub async fn send_request(&self, builder: RequestBuilder) -> Result<Response, HttpClientError> {
//         self.execute_request_with_circuit_breaker(|| {
//             Box::pin(async {
//                 builder.send().await.map_err(|e| {
//                     if e.is_timeout() {
//                         HttpClientError::Timeout
//                     } else {
//                         HttpClientError::NetworkError(e.to_string())
//                     }
//                 })
//             })
//         })
//         .await
//     }
// }
//
// struct HttpRequestBuilder<
//     'a,
//     POLICY: FailurePolicy + Send + Sync,
//     INSTRUMENT: Instrument + Send + Sync,
// > {
//     client: &'a FailsafeClient<POLICY, INSTRUMENT>,
//     builder: RequestBuilder,
// }
//
// impl<'a, POLICY: FailurePolicy + Send + Sync, INSTRUMENT: Instrument + Send + Sync>
//     HttpRequestBuilder<'a, POLICY, INSTRUMENT>
// {
//     pub fn new(client: &'a FailsafeClient<POLICY, INSTRUMENT>, builder: RequestBuilder) -> Self {
//         Self { client, builder }
//     }
//
//     pub fn header(mut self, key: &str, value: &str) -> Self {
//         self.builder = self.builder.header(key, value);
//         self
//     }
//
//     pub fn token(mut self, token: &str) -> Self {
//         self.builder = self.builder.bearer_auth(token);
//         self
//     }
//
//     pub fn timeout(mut self, duration: Duration) -> Self {
//         self.builder = self.builder.timeout(duration);
//         self
//     }
//
//     pub async fn send(self) -> Result<Response, HttpClientError> {
//         self.client.send_request(self.builder).await
//     }
// }
//
// #[derive(Error, Debug)]
// pub enum HttpClientError {
//     #[error("network error: {0}")]
//     NetworkError(String),
//     #[error("timeout error")]
//     Timeout,
//     #[error("circuit breaker is open")]
//     CircuitBreakerOpen,
// }
//
// #[cfg(test)]
// mod tests {
//     use failsafe::{backoff, failure_policy};
//     use tokio::runtime::Runtime;
//
//     use super::*;
//
//     #[test]
//     fn test_failsafe_client_new() {
//         let client = ArcClient::default();
//         let config = Config::new();
//         FailsafeClient::new(client, config);
//     }
//
//     #[test]
//     fn test_failsafe_client_request() {
//         let client = ArcClient::default();
//         let config = Config::new();
//         let failsafe_client = FailsafeClient::new(client, config);
//         failsafe_client
//             .get("https://api.example.com/data")
//             .token("your_token")
//             .header("Custom-Header", "CustomValue");
//     }
//
//     #[test]
//     fn test_http_request_builder_new() {
//         let client = ArcClient::default();
//         let config = Config::new();
//         let failsafe_client = FailsafeClient::new(client, config);
//         let _builder = failsafe_client.get("https://api.example.com/data");
//     }
//
//     #[test]
//     fn test_circuit_breaker_allows_request() {
//         let client = ArcClient::default();
//         let config = Config::new();
//         let failsafe_client = FailsafeClient::new(client, config);
//         let rt = Runtime::new().unwrap();
//         rt.block_on(async {
//             let response = failsafe_client.get("https://httpbin.org/get").send().await;
//             assert!(response.is_ok(), "Request should succeed");
//         });
//     }
//
//     #[test]
//     fn test_circuit_breaker_rejects_request() {
//         let client = ArcClient::default();
//         let b = backoff::equal_jittered(Duration::from_secs(1), Duration::from_secs(10));
//         let config = Config::new().failure_policy(
//             SuccessRateOverTimeWindow::default()
//                 .or_else(failure_policy::consecutive_failures(3, b)),
//         );
//         let failsafe_client = FailsafeClient::new(client, config);
//         let rt = Runtime::new().unwrap();
//         rt.block_on(async {
//             for _ in 0..4 {
//                 let _ = failsafe_client.get("https://nonexistent.url").send().await;
//             }
//             let response = failsafe_client.get("https://httpbin.org/get").send().await;
//             assert!(response.is_err(), "Request should be rejected by circuit breaker");
//         });
//     }
//
//     // #[test]
//     // fn test_http_request_builder_methods() {
//     //     let client = ArcClient::default();
//     //     let config = Config::new();
//     //     let failsafe_client = FailsafeClient::new(client, config);
//     //     let rt = Runtime::new().unwrap();
//     //     rt.block_on(async {
//     //         let builder = failsafe_client
//     //             .get("https://httpbin.org/get")
//     //             .header("Header-Name", "Header-Value")
//     //             .token("your_token")
//     //             .timeout(Duration::from_secs(10));
//     //         assert_eq!(builder.builder.headers().get("Header-Name").unwrap(), "Header-Value");
//     //         assert_eq!(
//     //             builder.builder.headers().get("Authorization").unwrap(),
//     //             "Bearer your_token"
//     //         );
//     //         assert_eq!(builder.builder.timeout().unwrap(), Duration::from_secs(10));
//     //     });
//     // }
//
//     #[test]
//     fn test_failsafe_client_default() {
//         let client = FailsafeClient::default();
//         let rt = Runtime::new().unwrap();
//         rt.block_on(async {
//             let response = client.get("https://httpbin.org/get").send().await;
//             assert!(response.is_ok(), "Request should succeed");
//         });
//     }
// }
//
// #[tokio::main]
// async fn main() {
//     // Initialize the tracing subscriber for structured logging
//     tracing_subscriber::fmt::init();
//
//     let client = FailsafeClient::default();
//
//     match client
//         .get("https://api.example.com/data")
//         .token("your_token")
//         .header("Custom-Header", "CustomValue")
//         .timeout(Duration::from_secs(30))
//         .send()
//         .await
//     {
//         Ok(response) => {
//             info!("Request successful: {:?}", response);
//         }
//         Err(e) => {
//             error!("Request failed: {:?}", e);
//         }
//     }
// }
