
use std::path::PathBuf;
#[tokio::main]
async fn main() {
    println!("cargo:rerun-if-changed=proto/config.yaml");
    let tonic_config = util_proto::load_tonic_config(
        format!("{}/proto/config.yaml", env!("CARGO_MANIFEST_DIR")).as_str(),
    );
    let out_dir = PathBuf::from(std::env::var("OUT_DIR").unwrap());
    tonic_config
        .clone()
        .file_descriptor_set_path(out_dir.join("gg.auth.bin"))
        .compile(&["proto/v1/rustproof.proto"], &["proto"])
        .expect("Failed to compile proto files");
}
