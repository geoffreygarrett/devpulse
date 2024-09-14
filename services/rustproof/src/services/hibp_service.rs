use reqwest::{Client, Error as ReqwestError, StatusCode};
use serde::Deserialize;
use sha1::{Digest, Sha1};
use std::collections::HashMap;
use std::fmt::Write;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum HIBPError {
    #[error("HTTP error: {0}")]
    Http(#[from] ReqwestError),
    #[error("Authorization error: {0}")]
    Unauthorized(String),
    #[error("API returned an error: {0}")]
    Api(String),
    #[error("Too many requests: rate limit exceeded, retry after {retry_after} seconds")]
    RateLimited { retry_after: u32 },
    #[error("Service unavailable")]
    ServiceUnavailable,
    #[error("Password found in breach {count} times")]
    PasswordBreached { count: u32 },
    #[error("Unknown error")]
    Unknown,
}

#[derive(Deserialize, Debug)]
pub struct Breach {
    pub name: String,
    pub title: String,
    pub domain: String,
    pub breach_date: String,
    pub added_date: String,
    pub pwn_count: u64,
    pub description: String,
    pub data_classes: Vec<String>,
    pub is_verified: bool,
    pub is_sensitive: bool,
}

pub struct HIBPService {
    client: Client,
    api_key: Option<String>,
    user_agent: String,
}

impl HIBPService {
    pub fn new(api_key: Option<String>, user_agent: String) -> Self {
        Self {
            client: Client::new(),
            api_key,
            user_agent,
        }
    }

    async fn request<T: for<'de> Deserialize<'de>>(
        &self,
        endpoint: &str,
        params: Option<&HashMap<&str, &str>>,
    ) -> Result<T, HIBPError> {
        let mut request = self.client.get(endpoint).header("User-Agent", &self.user_agent);

        if let Some(key) = &self.api_key {
            request = request.header("hibp-api-key", key);
        }

        if let Some(params) = params {
            request = request.query(params);
        }

        let response = request.send().await?;

        match response.status() {
            StatusCode::OK => {
                let parsed = response.json::<T>().await?;
                Ok(parsed)
            }
            StatusCode::UNAUTHORIZED => Err(HIBPError::Unauthorized(
                "Invalid or missing API key".to_string(),
            )),
            StatusCode::TOO_MANY_REQUESTS => {
                let retry_after = response
                    .headers()
                    .get("retry-after")
                    .and_then(|h| h.to_str().ok())
                    .and_then(|s| s.parse::<u32>().ok())
                    .unwrap_or(60); // Default to 60 seconds if not specified
                Err(HIBPError::RateLimited { retry_after })
            }
            StatusCode::SERVICE_UNAVAILABLE => Err(HIBPError::ServiceUnavailable),
            _ => Err(HIBPError::Api(response.text().await?)),
        }
    }

    pub async fn get_breaches_for_account(
        &self,
        account: &str,
    ) -> Result<Vec<Breach>, HIBPError> {
        let endpoint = format!("https://haveibeenpwned.com/api/v3/breachedaccount/{}", account);
        let breaches: Vec<Breach> = self.request(&endpoint, None).await?;
        Ok(breaches)
    }

    pub async fn check_password(&self, password: &str) -> Result<(), HIBPError> {
        let sha1 = Sha1::digest(password.as_bytes());
        let sha1_hex = sha1.iter().map(|b| format!("{:02X}", b)).collect::<String>();

        let prefix = &sha1_hex[0..5];
        let suffix = &sha1_hex[5..];

        let url = format!("https://api.pwnedpasswords.com/range/{}", prefix);
        let response = self.client.get(&url).send().await?;

        match response.status() {
            StatusCode::OK => {
                for line in response.text().await?.lines() {
                    let parts: Vec<&str> = line.split(':').collect();
                    if parts.len() == 2 && parts[0] == suffix {
                        let count: u32 = parts[1].trim().parse().map_err(|_| HIBPError::Unknown)?;
                        return Err(HIBPError::PasswordBreached { count });
                    }
                }
                Ok(())
            }
            StatusCode::SERVICE_UNAVAILABLE => Err(HIBPError::ServiceUnavailable),
            StatusCode::TOO_MANY_REQUESTS => {
                let retry_after = response
                    .headers()
                    .get("retry-after")
                    .and_then(|h| h.to_str().ok())
                    .and_then(|s| s.parse::<u32>().ok())
                    .unwrap_or(60);
                Err(HIBPError::RateLimited { retry_after })
            }
            _ => Err(HIBPError::Api(response.text().await?)),
        }
    }
}
//
// #[cfg(test)]
// mod tests {
//     use super::*;
//     use httpmock::Method::GET;
//     use httpmock::MockServer;
//     use serde_json::json;
//
//     #[tokio::test]
//     async fn test_check_password_safe() {
//         let mock_server = MockServer::start();
//
//         mock_server.mock(|when, then| {
//             when.method(GET)
//                 .path("/range/21BD1");
//             then.status(200)
//                 .body("0018A45C4D1DEF81644B54AB7F969B88D65:1\n011053FD0102E94D6AE2F8B83D76FAF94F6:2");
//         });
//
//         let hibp_service = HIBPService::new(None, "test-agent".to_string());
//
//         let result = hibp_service.check_password("password").await;
//         assert!(result.is_ok());
//     }
//
//     #[tokio::test]
//     async fn test_check_password_breached() {
//         let mock_server = MockServer::start();
//
//         mock_server.mock(|when, then| {
//             when.method(GET)
//                 .path("/range/21BD1");
//             then.status(200)
//                 .body("5C80B453D868DBB01A145BE3B526D47644A:10");
//         });
//
//         let hibp_service = HIBPService::new(None, "test-agent".to_string());
//
//         let result = hibp_service.check_password("password").await;
//         assert!(matches!(result, Err(HIBPError::PasswordBreached { count: 10 })));
//     }
//
//     #[tokio::test]
//     async fn test_get_breaches_for_account() {
//         let mock_server = MockServer::start();
//
//         mock_server.mock(|when, then| {
//             when.method(GET)
//                 .path("/api/v3/breachedaccount/test@example.com");
//             then.status(200)
//                 .json_body(json!([{
//                     "name": "Adobe",
//                     "title": "Adobe",
//                     "domain": "adobe.com",
//                     "breach_date": "2013-10-04",
//                     "added_date": "2013-12-04T00:00Z",
//                     "pwn_count": 152445165,
//                     "description": "In October 2013, 153 million Adobe accounts were breached...",
//                     "data_classes": ["Email addresses", "Passwords"],
//                     "is_verified": true,
//                     "is_sensitive": false,
//                 }]));
//         });
//
//         let hibp_service = HIBPService::new(Some("fake-key".to_string()), "test-agent".to_string());
//
//         let breaches = hibp_service.get_breaches_for_account("test@example.com").await.unwrap();
//         assert_eq!(breaches.len(), 1);
//         assert_eq!(breaches[0].name, "Adobe");
//     }
// }
