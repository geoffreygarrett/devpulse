use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub struct Schema {
    pub schema_version: String,
    pub type_definitions: Vec<TypeDefinition>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TypeDefinition {
    pub type_name: String,
    pub relations: HashMap<String, Vec<RelationTarget>>,
    pub metadata: Option<Metadata>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct RelationTarget {
    pub target_type: String,
    pub via_relation: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Metadata {
    pub relations_metadata: HashMap<String, RelationMetadata>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct RelationMetadata {
    pub directly_related_user_types: Vec<RelationTarget>,
}
