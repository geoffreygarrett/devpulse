#![allow(unused_imports)]
#![allow(clippy::too_many_arguments)]

extern crate reqwest;
extern crate serde;
extern crate serde_json;
extern crate url;

pub mod apis;
pub mod models;

pub mod prelude {
    pub use crate::apis::configuration::Configuration;
    pub use crate::apis::AzureClient;
}
