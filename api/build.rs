use std::fs;
use std::path::Path;

use tonic_build::Builder as TonicConfig;

use serde::{Deserialize, Serialize};

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
    DeriveBuilder,
    Documentation,
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

fn apply_configurations(mut tonic_config: TonicConfig, config: &Config) -> TonicConfig {
    for derive in &config.derives.message {
        if cfg_matches(&derive.condition) {
            tonic_config = tonic_config
                .type_attribute(&derive.target, &format!("#[derive({})]", derive.derive));
        }
    }

    for attribute in &config.attributes.message {
        if cfg_matches(&attribute.condition) {
            tonic_config = tonic_config.type_attribute(&attribute.target, &attribute.attribute);
        }
    }

    for derive in &config.derives.all {
        if cfg_matches(&derive.condition) {
            tonic_config = tonic_config
                .type_attribute(&derive.target, &format!("#[derive({})]", derive.derive));
        }
    }

    for attribute in &config.attributes.all {
        if cfg_matches(&attribute.condition) {
            tonic_config = tonic_config.type_attribute(&attribute.target, &attribute.attribute);
        }
    }

    tonic_config
}

fn cfg_matches(condition: &Option<String>) -> bool {
    match condition {
        Some(cond) => match cond.as_str() {
            "server" => cfg!(feature = "server"),
            "client" => cfg!(feature = "client"),
            "transport" => cfg!(feature = "transport"),
            "derive_builder" => cfg!(feature = "derive_builder"),
            "documentation" => cfg!(feature = "documentation"),
            _ => true,
        },
        None => true,
    }
}

fn compile_protos(tonic_config: &TonicConfig) {
    tonic_config
        .clone()
        .compile(
            &[
                "proto/common.proto",
                "proto/commit_range_analysis.proto",
                "proto/developer_performance.proto",
                "proto/errors.proto",
                "proto/server.proto",
                "proto/system_health.proto",
            ],
            &["proto/"],
        )
        .expect("Failed to compile proto files");
}
