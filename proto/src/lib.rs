use std::io::{Read, Write};

mod core;

pub mod auth {
    include!(concat!(env!("OUT_DIR"), "/auth.rs"));
}

pub mod commit {
    include!(concat!(env!("OUT_DIR"), "/commit.rs"));
}

#[cfg(feature="core")]
pub mod core_models {
    pub use core::*;
    use crate::core;
}

#[cfg(any(feature = "core", feature = "api"))]
pub mod api_models {
    // API models here...
}

#[cfg(feature = "dal")]
pub mod dal_models {
    // Data Access Layer models here...
}

#[cfg(feature = "external_github")]
pub mod github_models {
    // GitHub models here...
}

#[cfg(feature = "external_azure")]
pub mod azure_models {
    // Azure models here...
}

// pub mod bpe {
//     include!(concat!(env!("OUT_DIR"), "/bpe.rs"));
// }

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

#[cfg(test)]
mod tests {
    //
    // use super::bpe::{
    //     BpeTokens, DetokenizerRequest, DetokenizerResponse, TokenizerRequest, TokenizerResponse,
    // };
    // use super::load_bpe_tokens;
    //
    // #[test]
    // fn test_tokenizer_request_serialization() {
    //     let request = TokenizerRequest {
    //         text: "Hello, world!".to_string(),
    //     };
    //
    //     // Serialize the request to bytes
    //     let mut buf = Vec::new();
    //     request.encode(&mut buf).unwrap();
    //     println!("Serialized TokenizerRequest: {:?}", buf);
    //
    //     // Deserialize the bytes back to a request
    //     let decoded_request = TokenizerRequest::decode(&buf as &[u8]).unwrap();
    //     println!("Deserialized TokenizerRequest: {:?}", decoded_request);
    //
    //     assert_eq!(decoded_request.text, "Hello, world!");
    // }

    // #[test]
    // fn test_tokenizer_response_serialization() {
    //     let tokens = vec![1, 2];
    //
    //     let response = TokenizerResponse { token_ids: tokens };
    //
    //     // Serialize the response to bytes
    //     let mut buf = Vec::new();
    //     response.encode(&mut buf).unwrap();
    //     println!("Serialized TokenizerResponse: {:?}", buf);
    //
    //     // Deserialize the bytes back to a response
    //     let decoded_response = TokenizerResponse::decode(&buf as &[u8]).unwrap();
    //     println!("Deserialized TokenizerResponse: {:?}", decoded_response);
    //
    //     assert_eq!(decoded_response.token_ids, vec![1, 2]);
    // }
    //
    // #[test]
    // fn test_detokenizer_request_serialization() {
    //     let tokens = vec![1, 2];
    //
    //     let request = DetokenizerRequest { token_ids: tokens };
    //
    //     // Serialize the request to bytes
    //     let mut buf = Vec::new();
    //     request.encode(&mut buf).unwrap();
    //     println!("Serialized DetokenizerRequest: {:?}", buf);
    //
    //     // Deserialize the bytes back to a request
    //     let decoded_request = DetokenizerRequest::decode(&buf as &[u8]).unwrap();
    //     println!("Deserialized DetokenizerRequest: {:?}", decoded_request);
    //
    //     assert_eq!(decoded_request.token_ids, vec![1, 2]);
    // }
    //
    // #[test]
    // fn test_detokenizer_response_serialization() {
    //     let response = DetokenizerResponse {
    //         text: "Hello, world!".to_string(),
    //     };
    //
    //     // Serialize the response to bytes
    //     let mut buf = Vec::new();
    //     response.encode(&mut buf).unwrap();
    //     println!("Serialized DetokenizerResponse: {:?}", buf);
    //
    //     // Deserialize the bytes back to a response
    //     let decoded_response = DetokenizerResponse::decode(&buf as &[u8]).unwrap();
    //     println!("Deserialized DetokenizerResponse: {:?}", decoded_response);
    //
    //     assert_eq!(decoded_response.text, "Hello, world!");
    // }
    //
    // #[test]
    // fn test_load_bpe_tokens() {
    //     // Failing test
    //     let bpe_map = load_bpe_tokens("bpe_tokens.bin");
    //     for i in 1..=1000 {
    //         println!("{}: {}", i, bpe_map.get(&i).unwrap());
    //     }
    //     // assert_eq!(bpe_map.get(&1).unwrap(), "Hello");
    //     // assert_eq!(bpe_map.get(&2).unwrap(), "world");
    // }
}
