use std::env;
use std::path::Path;

use dotenv::dotenv;
use tracing::{debug, error, info};
use tracing_subscriber;

pub(crate) use auto_route::*;

pub fn convert_openapi_to_axum_path(openapi_path: &str) -> String {
    let mut axum_path = String::from(openapi_path);
    while let Some(start) = axum_path.find('{') {
        if let Some(end) = axum_path.find('}') {
            let param = &axum_path[start + 1..end];
            axum_path.replace_range(start..=end, &format!(":{}", param));
        } else {
            break;
        }
    }
    axum_path
}

pub(crate) mod auto_route {
    pub(crate) use axum_typed_routing::TypedRouter;

    pub(crate) use auto_route::route;
}

pub(crate) struct CertificatePaths {
    pub(crate) cert_path: String,
    pub(crate) key_path: String,
    pub(crate) ca_cert_path: String,
}

impl CertificatePaths {
    pub(crate) fn new() -> Self {
        let cert_path =
            Self::get_path("DEVPULSE_CERT_PATH", "DEVPULSE_CERT", "certs/server.pem", "cert");
        let key_path =
            Self::get_path("DEVPULSE_KEY_PATH", "DEVPULSE_CERT", "certs/server.key", "key");
        let ca_cert_path =
            Self::get_path("DEVPULSE_CA_CERT_PATH", "DEVPULSE_CERT", "certs/my_ca.pem", "CA cert");

        Self {
            cert_path,
            key_path,
            ca_cert_path,
        }
    }

    fn get_path(var: &str, base_var: &str, default: &str, name: &str) -> String {
        let path = env::var(var)
            .or_else(|_| {
                env::var(base_var).map(|p| {
                    let constructed_path = format!("{}/{}", p, default);
                    debug!("Using {} to find {} path: {}", base_var, name, constructed_path);
                    constructed_path
                })
            })
            .unwrap_or_else(|_| {
                let default_path = default.to_string();
                debug!("Using default {} path: {}", name, default_path);
                default_path
            });

        if !Path::new(&path).exists() {
            error!("File not found: {}", path);
            // panic!("File not found: {}", path);
        }

        info!("{} path: {}", name, path);
        path
    }
}
