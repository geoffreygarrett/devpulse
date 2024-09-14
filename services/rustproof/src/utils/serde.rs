use secrecy::Secret;
use serde::Serializer;
pub(crate) fn default_10_u64() -> u64 {
    10
}
pub(crate) fn default_0_u64() -> u64 { 0 }
pub(crate) fn default_900_u64() -> u64 {
    900
}
pub(crate) fn default_6_u8() -> u8 { 6 }
pub(crate) fn default_true() -> bool { true }
pub(crate) fn default_false() -> bool {
    false
}

pub fn default_http_port() -> u16 {
    8081
}

pub fn default_grpc_port() -> u16 {
    50051
}

pub fn default_enable_http() -> bool {
    true
}

pub fn default_enable_grpc() -> bool {
    true
}

pub fn default_rate_limit_email_sent_hourly() -> u32 {
    20
}

pub fn default_api_external_url() -> Option<String> {
    Some("http://localhost:8081".to_string())
}

fn default_api_port() -> u16 {
    8081
}

pub(crate) fn default_rate_limit_header() -> String {
    "X-RateLimit-Limit".to_string()
}
pub(crate) fn serialize_secret_redacted<S>(secret: &Secret<String>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_str("[REDACTED]")
}

pub(crate) fn serialize_option_secret_redacted<S>(option_secret: &Option<Secret<String>>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match option_secret {
        Some(secret) => serializer.serialize_str("[REDACTED]"),
        None => serializer.serialize_none(),
    }
}
