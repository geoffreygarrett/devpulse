// pub use graph::{models::Entity, RelationGraph, RelationTuple};
// pub use encoding::{from_protobuf, to_protobuf, to_zanzibar_notation, from_zanzibar_notation};

pub mod conversion;
pub mod encoding;
pub mod errors;
pub mod graph;
pub mod index;
pub mod models;
pub mod parser;
pub mod parsing;
pub mod policy;
pub mod traits;
pub mod config;

pub mod zookie {
    tonic::include_proto!("zookie");
}

pub mod api {
    tonic::include_proto!("api");
}

pub mod config1 {
    tonic::include_proto!("config");
}

pub mod acl {
    tonic::include_proto!("acl");
}
