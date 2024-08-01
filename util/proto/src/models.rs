use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub derives: Option<DerivesConfig>,
    pub attributes: Option<AttributesConfig>,
    pub features: Option<FeaturesConfig>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DerivesConfig {
    pub message: Vec<DeriveEntry>,
    #[serde(rename = "enum")]
    pub enum_: Vec<DeriveEntry>,
    pub all: Vec<DeriveEntry>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AttributesConfig {
    pub message: Vec<AttributeEntry>,
    #[serde(rename = "enum")]
    pub enum_: Vec<AttributeEntry>,
    pub all: Vec<AttributeEntry>,
    pub field: Vec<AttributeEntry>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FeaturesConfig {
    pub server: Option<FeatureConfig>,
    pub client: Option<FeatureConfig>,
    pub transport: Option<FeatureConfig>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FeatureConfig {
    pub build_server: Option<bool>,
    pub build_client: Option<bool>,
    pub build_transport: Option<bool>,
    pub use_arc_self: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeriveEntry {
    pub target: String,
    pub derive: String,
    pub condition: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AttributeEntry {
    pub target: String,
    pub attribute: String,
    pub condition: Option<String>,
}

