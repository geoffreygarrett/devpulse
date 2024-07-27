use std::time::Duration;

use criterion::{black_box, Criterion, criterion_group, criterion_main};
use prost::Message;
use rusqlite::{Connection, params};
use serde::{Deserialize, Serialize};

include!(concat!(env!("OUT_DIR"), "/auth.rs"));

#[derive(Serialize, Deserialize)]
struct LoginRequestJson {
    username: String,
    password: String,
}

fn save_result(name: &str, value: f64) {
    let conn = Connection::open("benchmarks.db").unwrap();
    conn.execute(
        "INSERT INTO benchmark_results (name, value) VALUES (?1, ?2)",
        params![name, value],
    )
    .unwrap();
}

fn bench_protobuf_serialize(c: &mut Criterion) {
    let login_request = LoginRequest {
        username: "test_user".to_string(),
        password: "test_pass".to_string(),
    };

    c.bench_function("protobuf serialize", |b| {
        b.iter(|| {
            let mut buf = Vec::new();
            login_request.encode(&mut buf).unwrap();
            black_box(buf);
        })
    });

    c.bench_function("protobuf serialize save", |b| {
        b.iter_custom(|iters| {
            let start = std::time::Instant::now();
            for _ in 0..iters {
                let mut buf = Vec::new();
                login_request.encode(&mut buf).unwrap();
                black_box(buf);
            }
            let elapsed = start.elapsed();
            save_result("protobuf serialize", elapsed.as_nanos() as f64 / iters as f64);
            elapsed
        });
    });
}

fn bench_protobuf_deserialize(c: &mut Criterion) {
    let login_request = LoginRequest {
        username: "test_user".to_string(),
        password: "test_pass".to_string(),
    };

    let mut buf = Vec::new();
    login_request.encode(&mut buf).unwrap();

    c.bench_function("protobuf deserialize", |b| {
        b.iter(|| {
            let decoded_request = LoginRequest::decode(&buf as &[u8]).unwrap();
            black_box(decoded_request);
        })
    });

    c.bench_function("protobuf deserialize save", |b| {
        b.iter_custom(|iters| {
            let start = std::time::Instant::now();
            for _ in 0..iters {
                let decoded_request = LoginRequest::decode(&buf as &[u8]).unwrap();
                black_box(decoded_request);
            }
            let elapsed = start.elapsed();
            save_result("protobuf deserialize", elapsed.as_nanos() as f64 / iters as f64);
            elapsed
        });
    });
}

fn bench_protobuf_size(c: &mut Criterion) {
    let login_request = LoginRequest {
        username: "test_user".to_string(),
        password: "test_pass".to_string(),
    };

    c.bench_function("protobuf size", |b| {
        b.iter(|| {
            let mut buf = Vec::new();
            login_request.encode(&mut buf).unwrap();
            let size = buf.len();
            save_result("protobuf size", size as f64);
            black_box(size);
        })
    });
}

fn bench_json_serialize(c: &mut Criterion) {
    let login_request = LoginRequestJson {
        username: "test_user".to_string(),
        password: "test_pass".to_string(),
    };

    c.bench_function("json serialize", |b| {
        b.iter(|| {
            let json_data = serde_json::to_string(&login_request).unwrap();
            black_box(json_data);
        })
    });

    c.bench_function("json serialize save", |b| {
        b.iter_custom(|iters| {
            let start = std::time::Instant::now();
            for _ in 0..iters {
                let json_data = serde_json::to_string(&login_request).unwrap();
                black_box(json_data);
            }
            let elapsed = start.elapsed();
            save_result("json serialize", elapsed.as_nanos() as f64 / iters as f64);
            elapsed
        });
    });
}

fn bench_json_deserialize(c: &mut Criterion) {
    let login_request = LoginRequestJson {
        username: "test_user".to_string(),
        password: "test_pass".to_string(),
    };

    let json_data = serde_json::to_string(&login_request).unwrap();

    c.bench_function("json deserialize", |b| {
        b.iter(|| {
            let decoded_request: LoginRequestJson = serde_json::from_str(&json_data).unwrap();
            black_box(decoded_request);
        })
    });

    c.bench_function("json deserialize save", |b| {
        b.iter_custom(|iters| {
            let start = std::time::Instant::now();
            for _ in 0..iters {
                let decoded_request: LoginRequestJson = serde_json::from_str(&json_data).unwrap();
                black_box(decoded_request);
            }
            let elapsed = start.elapsed();
            save_result("json deserialize", elapsed.as_nanos() as f64 / iters as f64);
            elapsed
        });
    });
}

fn bench_json_size(c: &mut Criterion) {
    let login_request = LoginRequestJson {
        username: "test_user".to_string(),
        password: "test_pass".to_string(),
    };

    c.bench_function("json size", |b| {
        b.iter(|| {
            let json_data = serde_json::to_string(&login_request).unwrap();
            let size = json_data.len();
            save_result("json size", size as f64);
            black_box(size);
        })
    });
}

// Grouping benchmarks for Protobuf
criterion_group! {
    name = protobuf_benches;
    config = Criterion::default().measurement_time(Duration::new(10, 0));
    targets = bench_protobuf_serialize, bench_protobuf_deserialize, bench_protobuf_size
}

// Grouping benchmarks for JSON
criterion_group! {
    name = json_benches;
    config = Criterion::default().measurement_time(Duration::new(10, 0));
    targets = bench_json_serialize, bench_json_deserialize, bench_json_size
}

// Running all benchmarks
criterion_main!(protobuf_benches, json_benches);
