pub(crate) use adapters::traits::data_access::{DataAccess, DbPool};

pub mod adapters;
#[cfg(feature = "server")]
pub mod services;

pub mod proto {
    pub mod db_auth_v1 {
        tonic::include_proto!("db.auth.v1");
    }
}

