use std::path::PathBuf;

fn main() {
    println!("cargo:rerun-if-changed=migrations");
    println!("cargo:rerun-if-changed=proto/config.yaml");
    let tonic_config = util_proto::load_tonic_config(
        format!("{}/proto/config.yaml", env!("CARGO_MANIFEST_DIR")).as_str(),
    );
    let out_dir = PathBuf::from(std::env::var("OUT_DIR").unwrap());
    tonic_config
        .clone()
        .file_descriptor_set_path(out_dir.join("db.v1.bin"))
        .compile(
            &[
                // "proto/v1/auth.proto",
                "proto/v1/filter.proto",
                "proto/v1/account.proto",
                "proto/v1/refresh_token.proto",
                "proto/v1/types.proto",
            ],
            &["proto"],
        )
        .expect("Failed to compile proto files");
}
