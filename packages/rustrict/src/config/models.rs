use std::collections::HashMap;
use anyhow::Error;
use color_eyre::eyre::Context;
use color_eyre::Help;
use serde::{Deserialize, Serialize};
use tracing::{error, instrument, Level, span};

use super::*;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ModelConfig {
    pub schema: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Configuration {
    pub model: ModelConfig,
    pub types: HashMap<String, TypeConfig>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TypeConfig {
    #[serde(default)]
    pub(crate) relations: Option<HashMap<String, RelationConfig>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum RelationConfig {
    Relation(Relation),
    Conditional(ConditionalRelation),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum Relation {
    Direct(Vec<DirectRelation>),
    Indirect(IndirectRelation),
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(untagged)]
pub enum DirectRelation {
    TupleString(String),
    TypeRelation {
        #[serde(rename = "type")]
        type_: String,
        relation: String,
    },
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum IndirectRelation {
    Internal(String),
    External { relation: String, from: String },
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ConditionalRelation {
    #[serde(rename = "or")]
    pub(crate) union: Option<Vec<RelationVariant>>,
    #[serde(rename = "and")]
    pub(crate) intersection: Option<Vec<RelationVariant>>,
    #[serde(rename = "but_not")]
    pub(crate) exclusion: Option<Vec<RelationVariant>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum RelationVariant {
    Relation(Relation),
    Conditional(Box<ConditionalRelation>),
}
