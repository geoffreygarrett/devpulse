use std::collections::HashMap;
use std::fs::{self, File};
use std::io::{Read, Write};

use prost_build::Config;

// URL for downloading the BPE tokens
const BPE_URL: &str =
    "https://openaipublic.blob.core.windows.net/gpt-2/encodings/main/encoder.json";

// File paths
const BPE_PROTO: &str = "proto/bpe.proto";
const BPE_TOKENS_BIN: &str = "bpe_tokens.bin";

// Download and parse the BPE tokens from the given URL
fn download_and_parse_bpe() -> HashMap<String, u32> {
    let mut resp = reqwest::blocking::get(BPE_URL).unwrap();
    let mut content = String::new();
    resp.read_to_string(&mut content).unwrap();
    let bpe_map: HashMap<String, u32> = serde_json::from_str(&content).unwrap();
    bpe_map
}

// Generate the Protobuf file for BPE
fn generate_bpe_proto() {
    let proto_content = r#"
syntax = "proto3";

package bpe;

message BpeToken {
    uint32 id = 1;
    string token = 2;
}

message BpeTokens {
    repeated BpeToken tokens = 1;
}

message TokenizerRequest {
    string text = 1;
}

message TokenizerResponse {
    repeated uint32 token_ids = 1;
}

message DetokenizerRequest {
    repeated uint32 token_ids = 1;
}

message DetokenizerResponse {
    string text = 1;
}
"#;

    std::fs::write(BPE_PROTO, proto_content).unwrap();
}

// Serialize the BPE tokens to a binary file
fn serialize_bpe_tokens(bpe_map: &HashMap<String, u32>, file_path: &str) {
    let mut file = File::create(file_path).unwrap();
    for (token, id) in bpe_map {
        file.write_all(&id.to_be_bytes()).unwrap();
        let token_len = token.len() as u32;
        file.write_all(&token_len.to_be_bytes()).unwrap();
        file.write_all(token.as_bytes()).unwrap();
    }
}

fn main() {
    println!(
        "cargo:rerun-if-changed={}/build/*",
        std::env::var("CARGO_MANIFEST_DIR").unwrap()
    );
    // // Step 1: Download and parse BPE tokens
    // let bpe_map = download_and_parse_bpe();

    // // Step 2: Generate Protobuf file
    // generate_bpe_proto();

    // // Step 3: Serialize BPE tokens to binary file
    // serialize_bpe_tokens(&bpe_map, BPE_TOKENS_BIN);

    // Step 4: Configure and compile Protobuf files
    let mut config = Config::new();
    config.enum_attribute(".", "#[derive(serde::Serialize, serde::Deserialize)]");
    config.message_attribute(
        ".",
        "#[derive(serde::Serialize, serde::Deserialize, derive_builder::Builder)]",
    );
    // Ensure output directory exists
    let out_dir = std::env::var("OUT_DIR").unwrap();
    fs::create_dir_all(format!("{}/proto", out_dir)).unwrap();

    // Compile Protobuf files
    config.out_dir(&out_dir);
    config
        .compile_protos(
            &[
                "proto/auth.proto",
                "proto/commit.proto",
                // BPE_PROTO
            ],
            &["proto/"],
        )
        .unwrap();
}
