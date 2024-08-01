pub(crate) use adapters::traits::data_access::{DataAccess, DbPool};

pub mod adapters;
#[cfg(feature = "server")]
pub mod services;

// pub mod proto {
//     pub mod db_auth_v1 {
//         tonic::include_proto!("db.auth.v1");
//     }
//
//     pub mod db_filter_v1 {}
// }

pub mod filter {
    pub mod v1 {
        tonic::include_proto!("db.filter.v1");
    }
}

pub mod account {
    pub mod v1 {
        pub use crate::adapters::postgres::uuid::*;

        tonic::include_proto!("db.account.v1");
    }
}

pub mod types {
    pub mod v1 {
        tonic::include_proto!("db.types.v1");
    }
}

pub mod refresh_token {
    pub mod v1 {
        tonic::include_proto!("db.refresh_token.v1");
    }
}
