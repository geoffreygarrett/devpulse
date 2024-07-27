use prost::Message;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Message)]
pub struct Claims {
    #[prost(string, tag = "1")]
    pub sub: String,
    #[prost(uint64, tag = "2")]
    pub exp: u64,
    #[prost(string, repeated, tag = "3")]
    pub roles: Vec<String>,
    #[prost(string, optional, tag = "4")]
    pub org: Option<String>,
    #[prost(uint64, optional, tag = "5")]
    pub iat: Option<u64>,
    #[prost(string, optional, tag = "6")]
    pub iss: Option<String>,
    #[prost(string, optional, tag = "7")]
    pub aud: Option<String>,
    #[prost(string, optional, tag = "8")]
    pub jti: Option<String>,
}
