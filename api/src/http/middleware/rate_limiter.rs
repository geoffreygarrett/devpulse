// use std::{
//     collections::HashMap,
//     sync::{Arc, Mutex},
//     time::{Duration, Instant},
// };
//
// use tokio::sync::Semaphore;
// use tower::{Layer, Service};
// use std::task::{Context, Poll};
// use std::future::Future;
// use std::pin::Pin;
// use std::net::IpAddr;
// use std::fmt;
//
// const IP_REQUEST_LIMIT: usize = 2;
// const USER_REQUEST_LIMIT: usize = 2;
// const TOKEN_REFILL_INTERVAL: Duration = Duration::from_secs(60);
//
// #[derive(Debug)]
// pub struct RateLimiter {
//     ip_requests: Mutex<HashMap<IpAddr, usize>>,
//     user_requests: Mutex<HashMap<String, usize>>,
//     last_refill: Instant,
//     semaphore: Semaphore,
// }
//
// impl RateLimiter {
//     pub fn new() -> Arc<Self> {
//         Arc::new(Self {
//             ip_requests: Mutex::new(HashMap::new()),
//             user_requests: Mutex::new(HashMap::new()),
//             last_refill: Instant::now(),
//             semaphore: Semaphore::new(IP_REQUEST_LIMIT + USER_REQUEST_LIMIT),
//         })
//     }
//
//     pub fn check_rate_limited(&mut self, identifier: &str, ip_addr: IpAddr) -> Result<(), String> {
//         let now = Instant::now();
//         let mut ip_map = self.ip_requests.lock().unwrap();
//         let mut user_map = self.user_requests.lock().unwrap();
//
//         if now.duration_since(self.last_refill) >= TOKEN_REFILL_INTERVAL {
//             ip_map.clear();
//             user_map.clear();
//             self.last_refill = now;
//         }
//
//         let ip_tokens = ip_map.entry(ip_addr).or_insert(USER_REQUEST_LIMIT);
//         let user_tokens = user_map.entry(identifier.to_string()).or_insert(IP_REQUEST_LIMIT);
//
//         if *ip_tokens == 0 || *user_tokens == 0 {
//             Err("Rate limit exceeded".to_string())
//         } else {
//             *ip_tokens -= 1;
//             *user_tokens -= 1;
//             Ok(())
//         }
//     }
// }
//
// #[derive(Clone)]
// pub struct RateLimitLayer {
//     limiter: Arc<RateLimiter>,
// }
//
// impl RateLimitLayer {
//     pub fn new(limiter: Arc<RateLimiter>) -> Self {
//         RateLimitLayer { limiter }
//     }
// }
//
// impl<S> Layer<S> for RateLimitLayer {
//     type Service = RateLimitService<S>;
//
//     fn layer(&self, service: S) -> Self::Service {
//         RateLimitService {
//             limiter: Arc::clone(&self.limiter),
//             service,
//         }
//     }
// }
//
// pub struct RateLimitService<S> {
//     limiter: Arc<RateLimiter>,
//     service: S,
// }
//
// impl<S, Request> Service<Request> for RateLimitService<S>
// where
//     S: Service<Request>,
//     S::Error: From<String>,  // Make sure S::Error can be constructed from String
//     Request: fmt::Debug + 'static,
// {
//     type Response = S::Response;
//     type Error = S::Error;
//     type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;
//
//     fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
//         self.service.poll_ready(cx)
//     }
//
//     fn call(&mut self, request: Request) -> Self::Future {
//         let mut limiter = self.limiter.clone();
//         let service = self.service.clone();
//
//         Box::pin(async move {
//             limiter.check_rate_limited("user123", IpAddr::from([127, 0, 0, 1]))
//                 .map_err(Into::into)
//                 .and_then(|_| service.call(request))
//         })
//     }
// }
