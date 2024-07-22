use tonic_build;

fn main() {
    tonic_build::compile_protos("proto/devpulse.proto").unwrap();
    // .out_dir("src/proto")
    // .compile(&["proto/devpulse.proto"], &["proto"])
    // .unwrap();
    // tonic_build::compile_protos("proto/devpulse.proto").unwrap();
}
