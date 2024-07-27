// use std::collections::HashMap;
// use std::fs::File;
// use std::io::{Read, Write};
//
// const BPE_URL: &str =
//     "https://openaipublic.blob.core.windows.net/gpt-2/encodings/main/encoder.json";
// const BPE_PROTO: &str = "proto/bpe.proto";
//
// pub fn download_and_parse_bpe() -> HashMap<String, u32> {
//     let mut resp = reqwest::blocking::get(BPE_URL).unwrap();
//     let mut content = String::new();
//     resp.read_to_string(&mut content).unwrap();
//     let bpe_map: HashMap<String, u32> = serde_json::from_str(&content).unwrap();
//     bpe_map
// }
//
// pub fn generate_bpe_proto() {
//     let proto_content = r#"
// syntax = "proto3";
//
// package bpe;
//
// message BpeToken {
//     uint32 id = 1;
//     string token = 2;
// }
//
// message BpeTokens {
//     repeated BpeToken tokens = 1;
// }
//
// message TokenizerRequest {
//     string text = 1;
// }
//
// message TokenizerResponse {
//     repeated uint32 token_ids = 1;
// }
//
// message DetokenizerRequest {
//     repeated uint32 token_ids = 1;
// }
//
// message DetokenizerResponse {
//     string text = 1;
// }
// "#;
//
//     std::fs::write(BPE_PROTO, proto_content).unwrap();
// }
//
// pub fn serialize_bpe_tokens(bpe_map: &HashMap<String, u32>, file_path: &str) {
//     let mut file = File::create(file_path).unwrap();
//     for (token, id) in bpe_map {
//         file.write_all(&id.to_be_bytes()).unwrap();
//         let token_len = token.len() as u32;
//         file.write_all(&token_len.to_be_bytes()).unwrap();
//         file.write_all(token.as_bytes()).unwrap();
//     }
// }
//
// pub fn load_bpe_tokens(file_path: &str) -> HashMap<u32, String> {
//     let mut file = File::open(file_path).unwrap();
//     let mut bpe_map = HashMap::new();
//     let mut buf = [0u8; 4];
//
//     while let Ok(_) = file.read_exact(&mut buf) {
//         let id = u32::from_be_bytes(buf);
//         file.read_exact(&mut buf).unwrap();
//         let token_len = u32::from_be_bytes(buf);
//         let mut token_bytes = vec![0u8; token_len as usize];
//         file.read_exact(&mut token_bytes).unwrap();
//         let token = String::from_utf8(token_bytes).unwrap();
//         bpe_map.insert(id, token);
//     }
//
//     bpe_map
// }
