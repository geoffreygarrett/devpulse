// use std::path::PathBuf;
//
// fn compile_protos(tonic_config: &TonicConfig) {
//     let out_dir = PathBuf::from(std::env::var("OUT_DIR").unwrap());
//     tonic_config
//         .clone()
//         .file_descriptor_set_path(out_dir.join("grpc.v1.bin"))
//         .compile(
//             &[
//                 "proto/common.proto",
//                 "proto/commit_range_analysis.proto",
//                 "proto/developer_performance.proto",
//                 "proto/errors.proto",
//                 "proto/server.proto",
//                 "proto/operational.proto",
//                 "proto/repository.proto",
//                 "proto/v1/health.proto",
//             ],
//             &["proto/"],
//         )
//         .expect("Failed to compile proto files");
// }
