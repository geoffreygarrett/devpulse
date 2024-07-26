use std::env;
use std::path::Path;
use std::process::Command;

// Constants for paths and the CLI tool
const OPENAPI_SPEC_PATH: &str = "./spec/api.github.com.extracted.json";
const OUTPUT_DIR: &str = "generated";
const TEMPLATE_DIR: &str = "./templates/rust-templates";
const OPENAPI_GENERATOR_CLI_PATH: &str = "openapi-generator";

fn generate_external_github_client() {
    let spec_path = env::current_dir().unwrap().join(OPENAPI_SPEC_PATH);
    let template_dir = env::current_dir().unwrap().join(TEMPLATE_DIR);
    if !spec_path.exists() {
        panic!("The specification file {:?} does not exist.", spec_path);
    }

    if !template_dir.exists() {
        panic!("The template directory {:?} does not exist.", template_dir);
    }

    let output_path = env::current_dir().unwrap().join(OUTPUT_DIR);
    if !output_path.exists() {
        std::fs::create_dir_all(&output_path).expect("Failed to create output directory");
    }

    println!("Running OpenAPI Generator...");
    println!("Spec path: {:?}", spec_path);
    println!("Output directory: {:?}", output_path);

    let generator_path = Command::new("which")
        .arg(OPENAPI_GENERATOR_CLI_PATH)
        .output()
        .expect("Failed to run `which` command");

    if generator_path.stdout.is_empty() {
        panic!("OpenAPI Generator CLI not found. Please install it.");
    }

    println!(
        "OpenAPI Generator CLI found at: {:?}",
        String::from_utf8_lossy(&generator_path.stdout)
    );

    let status = Command::new(OPENAPI_GENERATOR_CLI_PATH)
        .arg("generate")
        .arg("-i")
        .arg(spec_path.to_str().unwrap())
        .arg("-g")
        .arg("rust")
        .arg("-o")
        .arg(output_path.to_str().unwrap())
        .arg("--template-dir")
        .arg(template_dir.to_str().unwrap())
        .arg("--additional-properties=packageName=external_github,useSingleRequestParameter=true,deriveBuilder=true")
        .status()
        .expect("Failed to run OpenAPI Generator");

    println!(
        "Command executed: openapi-generator-cli generate -i {} -g rust -o {} --template-dir {} --additional-properties=packageName=external_github,useSingleRequestParameter=true,deriveBuilder=true",
        spec_path.display(),
        output_path.display(),
        TEMPLATE_DIR
    );

    if !status.success() {
        panic!("OpenAPI Generator failed with status: {}", status);
    }

    println!("OpenAPI Generator ran successfully");

    let out_dir = env::var("OUT_DIR").expect("OUT_DIR environment variable is not set");
    let src_path = output_path.join("src/lib.rs");
    let dest_path = Path::new(&out_dir).join("__external_github_client.rs");

    std::fs::copy(&src_path, &dest_path)
        .expect("Failed to copy generated file to output directory");
}

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed={}", OPENAPI_SPEC_PATH);
    generate_external_github_client();
}
