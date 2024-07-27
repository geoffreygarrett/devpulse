use std::time::Duration;

use criterion::{black_box, Criterion, criterion_group, criterion_main};
use hex::encode as hex_encode;
use prost::Message;
use serde::{Deserialize, Serialize};

include!(concat!(env!("OUT_DIR"), "/auth.rs"));

#[derive(Serialize, Deserialize)]
struct AuthRequestJson {
    username: String,
    password: String,
}

fn bench_protobuf(c: &mut Criterion) {
    let auth_request = AuthRequest {
        username: "test_user".to_string(),
        password: "test_pass".to_string(),
    };

    let mut buf = Vec::new();
    auth_request.encode(&mut buf).unwrap();
    println!("Protobuf Serialized Size: {}", buf.len());
    println!("Protobuf Serialized (Hex): {:?}", hex_encode(&buf));

    c.bench_function("protobuf serialize", |b| {
        b.iter(|| {
            let mut buf = Vec::new();
            auth_request.encode(&mut buf).unwrap();
            black_box(buf);
        })
    });

    c.bench_function("protobuf deserialize", |b| {
        b.iter(|| {
            let decoded_request = AuthRequest::decode(&buf as &[u8]).unwrap();
            black_box(decoded_request);
        })
    });

    c.bench_function("protobuf size", |b| {
        b.iter(|| {
            let mut buf = Vec::new();
            auth_request.encode(&mut buf).unwrap();
            black_box(buf.len());
        })
    });
}

fn bench_json(c: &mut Criterion) {
    let auth_request = AuthRequestJson {
        username: "test_user".to_string(),
        password: "test_pass".to_string(),
    };

    let json_data = serde_json::to_string(&auth_request).unwrap();
    println!("JSON Serialized Size: {}", json_data.len());
    println!("JSON Serialized: {}", json_data);

    c.bench_function("json serialize", |b| {
        b.iter(|| {
            let json_data = serde_json::to_string(&auth_request).unwrap();
            black_box(json_data);
        })
    });

    c.bench_function("json deserialize", |b| {
        b.iter(|| {
            let decoded_request: AuthRequestJson = serde_json::from_str(&json_data).unwrap();
            black_box(decoded_request);
        })
    });

    c.bench_function("json size", |b| {
        b.iter(|| {
            let json_data = serde_json::to_string(&auth_request).unwrap();
            black_box(json_data.len());
        })
    });
}

fn serialization_benchmark(c: &mut Criterion) {
    bench_protobuf(c);
    bench_json(c);
}

criterion_group! {
    name = benches;
    config = Criterion::default().measurement_time(Duration::new(10, 0));
    targets = serialization_benchmark
}

criterion_main!(benches);
