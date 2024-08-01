use crate::models::AttributeEntry;
use crate::models::DeriveEntry;
use crate::models::FeaturesConfig;
use crate::models::Config;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::Path;

use serde_yaml;
use tonic_build::Builder as TonicConfig;

pub fn load_tonic_config(path: &str) -> TonicConfig {
    let config_path = Path::new(path);
    let config_str = fs::read_to_string(config_path).expect("Failed to read config.yaml");
    serde_yaml::from_str::<Config>(&config_str)
        .expect("Failed to parse YAML")
        .into_tonic_config()

        .extern_path(".prost", "util_prost::prost")
}

impl From<Config> for TonicConfig {
    fn from(config: Config) -> Self {
        let mut tonic_config = tonic_build::configure();
        match &config.features {
            Some(features) => apply_features(&mut tonic_config, features),
            None => (),
        }
        apply_configurations(tonic_config, &config)
    }
}

impl Config {
    pub fn into_tonic_config(self) -> TonicConfig {
        self.into()
    }
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

fn type_attribute_wrapper(tonic_config: &mut TonicConfig, target: &str, attributes: &str) {
    *tonic_config = tonic_config.clone().type_attribute(target, attributes);
}

fn field_attribute_wrapper(tonic_config: &mut TonicConfig, target: &str, attributes: &str) {
    *tonic_config = tonic_config.clone().field_attribute(target, attributes);
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
    // Apply derives if they are present
    if let Some(derives) = &config.derives {
        accumulate_and_apply_derives(
            //
            &mut tonic_config,
            &derives.message,
            message_attribute_wrapper,
        );
        accumulate_and_apply_derives(
            //
            &mut tonic_config,
            &derives.all,
            enum_attribute_wrapper,
        );
    }

    // Apply message attributes if they are present
    if let Some(attributes) = &config.attributes {
        apply_attributes(
            //
            &mut tonic_config,
            &attributes.message,
            message_attribute_wrapper,
        );
        apply_attributes(
            //
            &mut tonic_config,
            &attributes.all,
            enum_attribute_wrapper,
        );
        apply_attributes(
            //
            &mut tonic_config,
            &attributes.field,
            field_attribute_wrapper,
        );
    }

    tonic_config
}
