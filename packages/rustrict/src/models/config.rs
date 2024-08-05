use std::collections::HashMap;

use serde::{Deserialize, Serialize};

/// Represents a direct userset in a userset rewrite rule.
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct DirectUserset {}

/// Represents a wildcard in a relation reference.
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct Wildcard {}

/// Represents an object relation, including an optional object and a relation.
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct ObjectRelation {
    #[serde(rename = "object", skip_serializing_if = "Option::is_none")]
    pub object: Option<String>,
    #[serde(rename = "relation")]
    pub relation: Option<String>,
}

/// Represents a reference to a relation, including type, relation, wildcard, and condition.
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct RelationReference {
    #[serde(rename = "type")]
    pub _type: String,
    #[serde(rename = "relation", skip_serializing_if = "Option::is_none")]
    pub relation: Option<String>,
    #[serde(rename = "wildcard", skip_serializing_if = "Option::is_none")]
    pub wildcard: Option<Wildcard>,
    #[serde(rename = "condition", skip_serializing_if = "Option::is_none")]
    pub condition: Option<String>,
}

/// Represents a tuple to userset transformation in a userset rewrite rule.
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct TupleToUserset {
    #[serde(rename = "tupleset")]
    pub tupleset: ObjectRelation,
    #[serde(rename = "computedUserset")]
    pub computed_userset: ObjectRelation,
}

/// Represents a userset rewrite rule with various possible components.
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct UsersetRewrite {
    #[serde(rename = "this", skip_serializing_if = "Option::is_none")]
    pub this: Option<DirectUserset>,
    #[serde(rename = "computedUserset", skip_serializing_if = "Option::is_none")]
    pub computed_userset: Option<ObjectRelation>,
    #[serde(rename = "tupleToUserset", skip_serializing_if = "Option::is_none")]
    pub tuple_to_userset: Option<TupleToUserset>,
    #[serde(rename = "union", skip_serializing_if = "Option::is_none")]
    pub union: Option<Vec<Box<UsersetRewrite>>>,
    #[serde(rename = "intersection", skip_serializing_if = "Option::is_none")]
    pub intersection: Option<Vec<Box<UsersetRewrite>>>,
    #[serde(rename = "exclusion", skip_serializing_if = "Option::is_none")]
    pub exclusion: Option<Vec<Box<UsersetRewrite>>>,
}

/// Represents the schema version of the model.
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct Schema {
    pub version: String,
}

/// Represents the configuration of the model, including schema and type definitions.
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct ModelConfig {
    pub schema: Schema,
    pub types: Vec<TypeDefinition>,
}

/// Provides source information for elements in the model.
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct SourceInfo {
    pub file: String,
    pub line: u32,
    pub line_end: u32,
    pub column: u32,
    pub column_end: u32,
}

/// Represents metadata for a relation, including directly related user types, module, and source information.
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct RelationMetadata {
    #[serde(rename = "directly_related_user_types")]
    pub directly_related_user_types: Option<Vec<RelationReference>>,
    #[serde(rename = "module")]
    pub module: Option<String>,
    #[serde(rename = "source_info")]
    pub source_info: Option<SourceInfo>,
}

/// Represents metadata for a type, including relations and other metadata.
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Metadata {
    #[serde(rename = "relations")]
    pub relations: HashMap<String, RelationMetadata>,
    #[serde(rename = "module")]
    pub module: Option<String>,
    #[serde(rename = "source_info")]
    pub source_info: Option<SourceInfo>,
}

/// Represents a type definition, including its relations and metadata.
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct TypeDefinition {
    #[serde(rename = "type")]
    pub _type: String,
    pub relations: Option<HashMap<String, UsersetRewrite>>,
    pub metadata: Option<Metadata>,
}
