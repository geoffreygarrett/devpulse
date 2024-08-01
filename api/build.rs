use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};
use tonic_build::Builder as TonicConfig;

#[derive(Debug, Serialize, Deserialize)]
struct Config {
    derives: DerivesConfig,
    attributes: AttributesConfig,
    features: FeaturesConfig,
}

#[derive(Debug, Serialize, Deserialize)]
struct DerivesConfig {
    message: Vec<DeriveEntry>,
    #[serde(rename = "enum")]
    enum_: Vec<DeriveEntry>,
    all: Vec<DeriveEntry>,
}

#[derive(Debug, Serialize, Deserialize)]
struct AttributesConfig {
    message: Vec<AttributeEntry>,
    #[serde(rename = "enum")]
    enum_: Vec<AttributeEntry>,
    all: Vec<AttributeEntry>,
}

#[derive(Debug, Serialize, Deserialize)]
struct FeaturesConfig {
    server: Option<FeatureConfig>,
    client: Option<FeatureConfig>,
    transport: Option<FeatureConfig>,
}

#[derive(Debug, Serialize, Deserialize)]
struct FeatureConfig {
    build_server: Option<bool>,
    build_client: Option<bool>,
    build_transport: Option<bool>,
    use_arc_self: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
struct DeriveEntry {
    target: String,
    derive: String,
    condition: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct AttributeEntry {
    target: String,
    attribute: String,
    condition: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
enum Feature {
    Server,
    Client,
    Transport,
    UseBuilder,
    Documentation,
}

impl From<Config> for TonicConfig {
    fn from(config: Config) -> Self {
        let mut tonic_config = tonic_build::configure();
        apply_features(&mut tonic_config, &config.features);
        apply_configurations(tonic_config, &config)
    }
}

impl Config {
    pub fn into_tonic_config(self) -> TonicConfig {
        self.into()
    }
}

fn main() {
    let config = load_config();
    let mut tonic_config = tonic_build::configure();

    apply_features(&mut tonic_config, &config.features);
    tonic_config = apply_configurations(tonic_config, &config);
    compile_protos(&tonic_config);
}

fn load_config() -> Config {
    let config_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("proto/config.yaml");
    let config_str = fs::read_to_string(config_path).expect("Failed to read config.yaml");
    serde_yaml::from_str(&config_str).expect("Failed to parse YAML")
}

fn apply_features(tonic_config: &mut TonicConfig, features: &FeaturesConfig) {
    if let Some(server) = &features.server {
        if server.build_server.unwrap_or(false) {
            *tonic_config = tonic_config.clone().build_server(true);
        }
        if server.use_arc_self.unwrap_or(false) {
            *tonic_config = tonic_config.clone().use_arc_self(true);
        }
    }

    if let Some(client) = &features.client {
        if client.build_client.unwrap_or(false) {
            *tonic_config = tonic_config.clone().build_client(true);
        }
    }

    if let Some(transport) = &features.transport {
        if transport.build_transport.unwrap_or(false) {
            *tonic_config = tonic_config.clone().build_transport(true);
        }
    }
}

// Assuming TonicConfig is `tonic_build::Builder`
fn message_attribute_wrapper(tonic_config: &mut TonicConfig, target: &str, attributes: &str) {
    *tonic_config = tonic_config.clone().message_attribute(target, attributes);
}

fn enum_attribute_wrapper(tonic_config: &mut TonicConfig, target: &str, attributes: &str) {
    *tonic_config = tonic_config.clone().enum_attribute(target, attributes);
}

// Adjusted to use mutable references
fn accumulate_and_apply_derives(
    tonic_config: &mut TonicConfig, derives: &[DeriveEntry],
    attribute_applier: fn(&mut TonicConfig, &str, &str),
) {
    let mut derive_map: HashMap<String, HashSet<String>> = HashMap::new();

    for derive in derives {
        if cfg_matches(&derive.condition) {
            derive_map
                .entry(derive.target.clone())
                .or_insert_with(HashSet::new)
                .insert(derive.derive.clone());
        }
    }

    for (target, derives) in derive_map {
        let derives_list: Vec<String> = derives.into_iter().collect();
        let derives_string = format!("#[derive({})]", derives_list.join(", "));
        attribute_applier(tonic_config, &target, &derives_string);
    }
}

fn apply_attributes(
    tonic_config: &mut TonicConfig, attributes: &[AttributeEntry],
    attribute_applier: fn(&mut TonicConfig, &str, &str),
) {
    for attribute in attributes {
        if cfg_matches(&attribute.condition) {
            attribute_applier(tonic_config, &attribute.target, &attribute.attribute);
        }
    }
}

fn apply_configurations(mut tonic_config: TonicConfig, config: &Config) -> TonicConfig {
    accumulate_and_apply_derives(
        &mut tonic_config,
        &config.derives.message,
        message_attribute_wrapper,
    );

    apply_attributes(&mut tonic_config, &config.attributes.message, message_attribute_wrapper);

    // Apply all-type derive attributes
    accumulate_and_apply_derives(&mut tonic_config, &config.derives.all, enum_attribute_wrapper);

    // Apply all-type attributes
    apply_attributes(&mut tonic_config, &config.attributes.all, enum_attribute_wrapper);

    tonic_config
}

fn cfg_matches(condition: &Option<String>) -> bool {
    match condition {
        Some(cond) => {
            match std::env::var(format!("CARGO_FEATURE_{}", cond.to_uppercase().replace("-", "_")))
            {
                Ok(_) => true,
                Err(_) => false,
            }
        }
        None => true,
    }
}

fn compile_protos(tonic_config: &TonicConfig) {
    let out_dir = PathBuf::from(std::env::var("OUT_DIR").unwrap());
    tonic_config
        .clone()
        .file_descriptor_set_path(out_dir.join("grpc.v1.bin"))
        .compile(
            &[
                "proto/common.proto",
                "proto/commit_range_analysis.proto",
                "proto/developer_performance.proto",
                "proto/errors.proto",
                "proto/server.proto",
                "proto/operational.proto",
                "proto/repository.proto",
                "proto/v1/health.proto",
            ],
            &["proto/"],
        )
        .expect("Failed to compile proto files");
}
