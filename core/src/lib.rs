extern crate devpulse_externals as externals;
pub use library::*;


pub(crate) mod github {
    pub use externals::external_github::*;
}

mod adapters;
pub(crate) mod analyzers;
mod clients;
pub(crate) mod commit;
pub(crate) mod library;
pub mod models;
pub(crate) mod pull_request;
pub(crate) mod repository;
pub mod services;
pub(crate) mod utils;

pub mod prelude {
    pub use commit_inspection::*;

    pub use crate::library::*;
}
