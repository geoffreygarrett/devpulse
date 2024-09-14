pub mod errors;
pub mod models;
pub mod config;
pub mod services;
pub mod adapter;
pub mod migration;
pub mod controllers;
pub mod repositories;
pub mod middleware;
pub mod email;
pub mod helper;
pub mod utils;

pub mod prelude {
    pub use jsonwebtoken::Validation;

    pub use crate::models::claims::RustProofClaims;
    // pub use crate::models::config::AuthConfig;
    pub use crate::models::keys::Keys;
}

pub mod v1 {
    tonic::include_proto!("com.gg.auth");
}
