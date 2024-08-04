use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct TypeDefinition {
    #[serde(rename = "type")]
    pub _type: String,
    #[serde(rename = "relations")]
    pub relations: Option<HashMap<String, Userset>>,
    #[serde(rename = "metadata")]
    pub metadata: Option<Metadata>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Userset {
    #[serde(rename = "this", skip_serializing_if = "Option::is_none")]
    pub this: Option<DirectUserset>,
    #[serde(rename = "computedUserset", skip_serializing_if = "Option::is_none")]
    pub computed_userset: Option<ObjectRelation>,
    #[serde(rename = "tupleToUserset", skip_serializing_if = "Option::is_none")]
    pub tuple_to_userset: Option<V1TupleToUserset>,
    #[serde(rename = "union", skip_serializing_if = "Option::is_none")]
    pub union: Option<Usersets>,
    #[serde(rename = "intersection", skip_serializing_if = "Option::is_none")]
    pub intersection: Option<Usersets>,
    #[serde(rename = "difference", skip_serializing_if = "Option::is_none")]
    pub difference: Option<V1Difference>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct DirectUserset {}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct ObjectRelation {
    #[serde(rename = "object")]
    pub object: Option<String>,
    #[serde(rename = "relation")]
    pub relation: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct V1TupleToUserset {
    #[serde(rename = "tupleset")]
    pub tupleset: ObjectRelation,
    #[serde(rename = "computedUserset")]
    pub computed_userset: ObjectRelation,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Usersets {
    #[serde(rename = "child")]
    pub child: Vec<Userset>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct V1Difference {
    #[serde(rename = "base")]
    pub base: Box<Userset>,
    #[serde(rename = "subtract")]
    pub subtract: Box<Userset>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Metadata {
    #[serde(rename = "relations")]
    pub relations: HashMap<String, RelationMetadata>,
    #[serde(rename = "module")]
    pub module: Option<String>,
    #[serde(rename = "source_info")]
    pub source_info: Option<SourceInfo>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct RelationMetadata {
    #[serde(rename = "directly_related_user_types")]
    pub directly_related_user_types: Option<Vec<RelationReference>>,
    #[serde(rename = "module")]
    pub module: Option<String>,
    #[serde(rename = "source_info")]
    pub source_info: Option<SourceInfo>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct RelationReference {
    #[serde(rename = "type")]
    pub _type: String,
    #[serde(rename = "relation")]
    pub relation: Option<String>,
    #[serde(rename = "wildcard")]
    pub wildcard: Option<Wildcard>,
    #[serde(rename = "condition")]
    pub condition: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Wildcard {}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct SourceInfo {
    #[serde(rename = "file", skip_serializing_if = "Option::is_none")]
    pub file: Option<String>,
}

#[cfg(test)]
mod tests {
    use serde_json;

    use super::*;

    #[test]
    fn test_serialization_deserialization() {
        let json_data = r#"
        {
            "type": "document",
            "relations": {
                "reader": {
                    "union": {
                        "child": [
                            { "this": {} },
                            { "computedUserset": { "object": "", "relation": "writer" } }
                        ]
                    }
                },
                "writer": { "this": {} }
            },
            "metadata": {
                "relations": {},
                "module": "example_module",
                "source_info": {}
            }
        }
        "#;

        let type_def: TypeDefinition = serde_json::from_str(json_data).unwrap();
        assert_eq!(type_def._type, "document");
        assert!(type_def.relations.is_some());
        assert!(type_def.metadata.is_some());

        let serialized = serde_json::to_string_pretty(&type_def).unwrap();
        println!("{}", serialized);

        // Deserialize the serialized JSON again to check for consistency
        let deserialized: TypeDefinition = serde_json::from_str(&serialized).unwrap();
        assert_eq!(type_def, deserialized);
    }
}
