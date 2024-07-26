// use std::path::Path;
// use std::process::Command;
// use std::{env, fs};

// // Constants for paths and the CLI tool
// const OPENAPI_SPEC_PATH: &str = "./spec/api.github.com.extracted.json";
// const OUTPUT_DIR: &str = "generated";
// const OPENAPI_GENERATOR_CLI_PATH: &str = "openapi-generator";
//
// fn prepare_generated_lib(output_path: &Path) {
//     // Create the output directory if it doesn't exist
//     if !output_path.exists() {
//         fs::create_dir_all(output_path).expect("Failed to create output directory");
//         // Optionally, create a basic Cargo.toml if your generator does not do it
//         let cargo_toml_contents = r#"
//             [package]
//             name = "external_github"
//             version = "0.1.0"
//             edition = "2018"
//
//             [dependencies]
//         "#;
//         fs::write(output_path.join("Cargo.toml"), cargo_toml_contents)
//             .expect("Failed to create Cargo.toml");
//     }
// }
//
// fn generate_external_github_client() {
//     // Build the full path to the OpenAPI specification file
//     prepare_generated_lib(&env::current_dir().unwrap().join(OUTPUT_DIR));
//     let spec_path = env::current_dir().unwrap().join(OPENAPI_SPEC_PATH);
//
//     // Check if the OpenAPI specification file exists
//     if !spec_path.exists() {
//         panic!("The specification file {:?} does not exist.", spec_path);
//     }
//
//     // Ensure the output directory exists or create it
//     let output_path = env::current_dir().unwrap().join(OUTPUT_DIR);
//     if !output_path.exists() {
//         std::fs::create_dir_all(&output_path).expect("Failed to create output directory");
//     }
//
//     // Debug prints
//     println!("Running OpenAPI Generator...");
//     println!("Spec path: {:?}", spec_path);
//     println!("Output directory: {:?}", output_path);
//
//     // Check if the OpenAPI Generator CLI is installed
//     let generator_path = Command::new("which")
//         .arg(OPENAPI_GENERATOR_CLI_PATH)
//         .output()
//         .expect("Failed to run `which` command");
//
//     if generator_path.stdout.is_empty() {
//         panic!("OpenAPI Generator CLI not found. Please install it.");
//     }
//
//     // Print the location of the OpenAPI Generator CLI if found
//     println!(
//         "OpenAPI Generator CLI found at: {:?}",
//         String::from_utf8_lossy(&generator_path.stdout)
//     );
//
//     // Run the OpenAPI Generator command
//     let status = Command::new(OPENAPI_GENERATOR_CLI_PATH)
//         .arg("generate")
//         .arg("-i")
//         .arg(spec_path.to_str().unwrap())
//         .arg("-g")
//         .arg("rust")
//         .arg("-o")
//         .arg(output_path.to_str().unwrap())
//         .arg("--additional-properties=useSingleRequestParameter=true,packageName=external_github")
//         .status()
//         .expect("Failed to run OpenAPI Generator");
//
//     // print command first
//
//     if !status.success() {
//         panic!("OpenAPI Generator failed with status: {}", status);
//     }
//
//     println!("OpenAPI Generator ran successfully");
//
//     // Copy the generated files to the desired output directory
//     let out_dir = env::var("OUT_DIR").expect("OUT_DIR environment variable is not set");
//     let src_path = output_path.join("src/lib.rs");
//     let dest_path = Path::new(&out_dir).join("__external_github_client.rs");
//
//     std::fs::copy(&src_path, &dest_path)
//         .expect("Failed to copy generated file to output directory");
// }

fn main() {
    // println!("cargo:rerun-if-changed=build.rs");
    // println!("cargo:rerun-if-changed={}", OPENAPI_SPEC_PATH);
    // generate_external_github_client();
}
