use std::collections::HashMap;
use serde::{Deserialize, Serialize};

/// Represents the various OAuth2 grant types.
///
/// This enum encapsulates the different grant types used in OAuth2 authentication flows.
/// Each variant corresponds to a specific grant type, with some additional information
/// provided in the associated comments.
///
/// Reference: https://fusionauth.io/articles/oauth/complete-list-oauth-grants
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "grant_type")]
pub enum OAuth2GrantType {
    /// The Authorization Code grant type.
    #[serde(rename = "authorization_code")]
    AuthorizationCode,

    /// The Implicit grant type.
    #[serde(rename = "implicit")]
    Implicit,

    /// The Resource Owner Password Credentials grant type.
    #[serde(rename = "password")]
    ResourceOwnerPassword,

    /// The Client Credentials grant type.
    #[serde(rename = "client_credentials")]
    ClientCredentials,

    /// The Refresh Token grant type.
    #[serde(rename = "refresh_token")]
    RefreshToken,

    /// The JWT Bearer grant type.
    #[serde(rename = "urn:ietf:params:oauth:grant-type:jwt-bearer")]
    JWTBearer,

    /// The Device Code grant type.
    #[serde(rename = "urn:ietf:params:oauth:grant-type:device_code")]
    DeviceCode,
}

impl OAuth2GrantType {
    /// Returns the string representation of the grant type as used in OAuth2 requests.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::AuthorizationCode => "authorization_code",
            Self::Implicit => "implicit",
            Self::ResourceOwnerPassword => "password",
            Self::ClientCredentials => "client_credentials",
            Self::RefreshToken => "refresh_token",
            Self::JWTBearer => "urn:ietf:params:oauth:grant-type:jwt-bearer",
            Self::DeviceCode => "urn:ietf:params:oauth:grant-type:device_code",
        }
    }

    /// Returns the required payload parameters for each grant type.
    pub fn payload(&self) -> HashMap<&'static str, &'static str> {
        match self {
            Self::AuthorizationCode => {
                let mut map = HashMap::new();
                map.insert("grant_type", "authorization_code");
                map.insert("code", "AUTHORIZATION_CODE");
                map.insert("redirect_uri", "REDIRECT_URI");
                map.insert("client_id", "CLIENT_ID");
                map
            }
            Self::Implicit => {
                let mut map = HashMap::new();
                map.insert("response_type", "token");
                map.insert("client_id", "CLIENT_ID");
                map.insert("redirect_uri", "REDIRECT_URI");
                map.insert("scope", "SCOPE");
                map.insert("state", "STATE");
                map
            }
            Self::ResourceOwnerPassword => {
                let mut map = HashMap::new();
                map.insert("grant_type", "password");
                map.insert("username", "USERNAME");
                map.insert("password", "PASSWORD");
                map.insert("scope", "SCOPE");
                map
            }
            Self::ClientCredentials => {
                let mut map = HashMap::new();
                map.insert("grant_type", "client_credentials");
                map.insert("scope", "SCOPE");
                map
            }
            Self::RefreshToken => {
                let mut map = HashMap::new();
                map.insert("grant_type", "refresh_token");
                map.insert("refresh_token", "REFRESH_TOKEN");
                map.insert("scope", "SCOPE");
                map
            }
            Self::JWTBearer => {
                let mut map = HashMap::new();
                map.insert("grant_type", "urn:ietf:params:oauth:grant-type:jwt-bearer");
                map.insert("assertion", "JWT_ASSERTION");
                map
            }
            Self::DeviceCode => {
                let mut map = HashMap::new();
                map.insert("grant_type", "urn:ietf:params:oauth:grant-type:device_code");
                map.insert("device_code", "DEVICE_CODE");
                map.insert("client_id", "CLIENT_ID");
                map
            }
        }
    }
}

/// Represents the various OAuth2 grants along with their associated payloads.
///
/// This enum encapsulates the different grant types used in OAuth2 authentication flows.
/// Each variant corresponds to a specific grant and includes a struct containing the
/// required payload parameters for that grant.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OAuth2Grant {
    /// The Authorization Code grant.
    AuthorizationCode(AuthorizationCodePayload),

    /// The Implicit grant.
    /// Note: This grant is no longer recommended for security reasons.
    Implicit(ImplicitPayload),

    /// The Resource Owner Password Credentials grant.
    ResourceOwnerPassword(ResourceOwnerPasswordPayload),

    /// The Client Credentials grant.
    ClientCredentials(ClientCredentialsPayload),

    /// The Refresh Token grant.
    RefreshToken(RefreshTokenPayload),

    /// The JWT Bearer grant.
    JWTBearer(JWTBearerPayload),

    /// The Device Code grant.
    DeviceCode(DeviceCodePayload),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AuthorizationCodePayload {
    pub code: String,
    pub redirect_uri: String,
    pub client_id: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ImplicitPayload {
    pub response_type: String,
    pub client_id: String,
    pub redirect_uri: String,
    pub scope: Option<String>,
    pub state: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResourceOwnerPasswordPayload {
    pub username: String,
    pub password: String,
    pub scope: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ClientCredentialsPayload {
    pub scope: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RefreshTokenPayload {
    pub refresh_token: String,
    pub scope: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct JWTBearerPayload {
    pub assertion: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeviceCodePayload {
    pub device_code: String,
    pub client_id: String,
}

impl OAuth2Grant {
    /// Returns the string representation of the grant as used in OAuth2 requests.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::AuthorizationCode(_) => "authorization_code",
            Self::Implicit(_) => "implicit",
            Self::ResourceOwnerPassword(_) => "password",
            Self::ClientCredentials(_) => "client_credentials",
            Self::RefreshToken(_) => "refresh_token",
            Self::JWTBearer(_) => "urn:ietf:params:oauth:grant-type:jwt-bearer",
            Self::DeviceCode(_) => "urn:ietf:params:oauth:grant-type:device_code",
        }
    }
}

// Example usage
fn main() {
    // Using OAuth2GrantType
    let grant_type = OAuth2GrantType::AuthorizationCode;
    println!("Grant Type: {}", grant_type.as_str());
    println!("Payload: {:?}", grant_type.payload());

    // Using OAuth2Grant
    let grant = OAuth2Grant::AuthorizationCode(AuthorizationCodePayload {
        code: "AUTHORIZATION_CODE".to_string(),
        redirect_uri: "https://example.com/callback".to_string(),
        client_id: "CLIENT_ID".to_string(),
    });

    println!("Grant: {}", grant.as_str());
    println!("Grant with Payload: {:?}", grant);
}