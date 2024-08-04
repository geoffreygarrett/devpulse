use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;
use tracing_error::SpanTrace;

mod dsl {

}

mod json {
    use std::collections::HashMap;

    use serde::Serialize;
    use serde_json::Value;

    #[derive(Debug, Serialize)]
    pub struct JsonSchema {
        schema_version: String,
        type_definitions: Vec<TypeDefinition>,
    }

    #[derive(Debug, Serialize)]
    struct TypeDefinition {
        #[serde(rename = "type")]
        type_name: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        relations: Option<HashMap<String, RelationDetail>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        metadata: Option<Metadata>,
    }

    #[derive(Debug, Serialize)]
    struct RelationDetail {
        #[serde(skip_serializing_if = "Option::is_none")]
        this: Option<Value>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "computedUserset")]
        computed_userset: Option<ComputedUserset>,
        #[serde(skip_serializing_if = "Option::is_none")]
        union: Option<Union>,
    }

    #[derive(Debug, Serialize)]
    struct ComputedUserset {
        object: String,
        relation: String,
    }

    #[derive(Debug, Serialize)]
    struct Union {
        child: Vec<RelationDetail>,
    }

    #[derive(Debug, Serialize)]
    struct Metadata {
        relations: HashMap<String, RelationMetadata>,
    }

    #[derive(Debug, Serialize)]
    struct RelationMetadata {
        directly_related_user_types: Vec<UserType>,
    }

    #[derive(Debug, Serialize)]
    struct UserType {
        #[serde(rename = "type")]
        type_name: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        relation: Option<String>,
    }
}


//     #[test]
//     fn test_deserialize_model() {
//         setup();
//         let yaml_data = r#"
//             model:
//               schema: 1.1
//
//             types:
//               user:
//                 # No relations needed for user as it's a base type
//
//               domain:
//                 relations:
//                   member:
//                     direct: [user]  # Directly relate users as members of a domain
//         "#;
//
//         let model: dsl::Configuration = parse_dsl(yaml_data);
//         assert_eq!(model.model.schema, "1.1");
//         assert_eq!(model.types.len(), 2);
//         assert!(model.types.contains_key("user"));
//         assert!(model.types.contains_key("domain"));
//     }
//
//     #[test]
//     fn test_deserialize_type_with_direct_relation() {
//         setup();
//         let yaml_data = r#"
//             model:
//               schema: 1.1
//
//             types:
//               user:
//                 # No relations needed for user as it's a base type
//
//               domain:
//                 relations:
//                   member:
//                     direct: [user]  # Directly relate users as members of a domain
//         "#;
//
//         let model: dsl::Configuration = parse_dsl(yaml_data);
//         let domain = model.types.get("domain").unwrap();
//         let member = domain.relations.as_ref().unwrap().get("member").unwrap();
//         match member {
//             dsl::Relation::Direct { direct: relations } => {
//                 assert_eq!(relations.len(), 1);
//                 assert_eq!(relations[0], dsl::DirectRelation::Type("user".to_string()));
//             }
//             _ => panic!("Expected direct relation"),
//         }
//     }
//
//     #[test]
//     fn test_deserialize_type_with_conditional_relation() {
//         setup();
//         let yaml_data = r#"
//             model:
//               schema: 1.1
//
//             types:
//               user:
//                 # No relations needed for user as it's a base type
//
//               document:
//                 relations:
//                   editor:
//                     or:
//                       - direct: [ "user" ]
//                       - from: "owner"
//         "#;
//
//         let model: dsl::Configuration = parse_dsl(yaml_data);
//         let document = model.types.get("document").unwrap();
//         let editor = document.relations.as_ref().unwrap().get("editor").unwrap();
//         match editor {
//             dsl::Relation::Conditional(cond) => {
//                 assert!(cond.or.is_some());
//                 let or_relations = cond.or.as_ref().unwrap();
//                 assert_eq!(or_relations.len(), 2);
//             }
//             _ => panic!("Expected conditional relation"),
//         }
//     }
//
//     #[test]
//     fn test_direct_relation_type() {
//         let yaml_data = r#"
//             - direct: [ "user" ]
//         "#;
//
//         let direct_relations: Vec<HashMap<String, Vec<DirectRelation>>> =
//             from_str(yaml_data).unwrap();
//         assert_eq!(direct_relations[0]["direct"][0], DirectRelation::Type("user".to_string()));
//     }
//
//     #[test]
//     fn test_direct_relation_type_relation() {
//         let yaml_data = r#"
//             - direct:
//                 - type_name: "domain"
//                   relation: "member"
//         "#;
//
//         let direct_relations: Vec<HashMap<String, Vec<DirectRelation>>> =
//             from_str(yaml_data).unwrap();
//         assert_eq!(
//             direct_relations[0]["direct"][0],
//             DirectRelation::TypeRelation {
//                 type_name: "domain".to_string(),
//                 relation: "member".to_string()
//             }
//         );
//     }
//
//     #[test]
//     fn test_mixed_direct_relations() {
//         let yaml_data = r#"
//             - direct:
//                 - "user"
//                 - type_name: "domain"
//                   relation: "member"
//         "#;
//
//         let direct_relations: Vec<HashMap<String, Vec<DirectRelation>>> =
//             from_str(yaml_data).unwrap();
//         assert_eq!(direct_relations[0]["direct"][0], DirectRelation::Type("user".to_string()));
//         assert_eq!(
//             direct_relations[0]["direct"][1],
//             DirectRelation::TypeRelation {
//                 type_name: "domain".to_string(),
//                 relation: "member".to_string()
//             }
//         );
//     }
//     #[test]
//     fn test_direct_relation_in_relation_enum() {
//         let yaml_data = r#"
//             member:
//               direct:
//                 - user
//         "#;
//
//         let relations: HashMap<String, Relation> = from_str(yaml_data).unwrap();
//         match &relations["member"] {
//             Relation::Direct { direct } => {
//                 assert_eq!(direct[0], DirectRelation::Type("user".to_string()));
//             }
//             _ => panic!("Expected direct relation"),
//         }
//     }
//
//     #[test]
//     fn test_combined_direct_and_conditional_relations() {
//         let yaml_data = r#"
//             member:
//               direct:
//                 - user
//             editor:
//               or:
//                 - direct:
//                     - user
//                 - from: owner
//         "#;
//
//         let relations: HashMap<String, Relation> = from_str(yaml_data).unwrap();
//
//         match &relations["member"] {
//             Relation::Direct { direct } => {
//                 assert_eq!(direct[0], DirectRelation::Type("user".to_string()));
//             }
//             _ => panic!("Expected direct relation"),
//         }
//
//         match &relations["editor"] {
//             Relation::Conditional(cond) => {
//                 assert!(cond.or.is_some());
//                 let or_relations = cond.or.as_ref().unwrap();
//                 match &or_relations[0] {
//                     RelationVariant::Direct { direct } => {
//                         assert_eq!(direct[0], DirectRelation::Type("user".to_string()));
//                     }
//                     _ => panic!("Expected direct relation variant"),
//                 }
//                 match &or_relations[1] {
//                     RelationVariant::Conditional(inner_cond) => {
//                         assert_eq!(inner_cond.from.as_ref().unwrap(), "owner");
//                     }
//                     _ => panic!("Expected conditional relation variant"),
//                 }
//             }
//             _ => panic!("Expected conditional relation"),
//         }
//     }
//     // #[test]
//     // fn test_combined_direct_and_conditional_relations() {
//     //     let yaml_data = r#"
//     //         member:
//     //           direct:
//     //             - user
//     //           editor:
//     //             or:
//     //               - direct:
//     //                   - user
//     //               - from: owner
//     //     "#;
//     //
//     //     let relations: HashMap<String, Relation> = from_str(yaml_data).unwrap();
//     //
//     //     match &relations["member"] {
//     //         Relation::Direct { direct } => {
//     //             assert_eq!(direct[0], DirectRelation::Type("user".to_string()));
//     //         }
//     //         _ => panic!("Expected direct relation"),
//     //     }
//     //
//     //     match &relations["editor"] {
//     //         Relation::Conditional(cond) => {
//     //             assert!(cond.or.is_some());
//     //             let or_relations = cond.or.as_ref().unwrap();
//     //             match &or_relations[0] {
//     //                 RelationVariant::Direct { direct } => {
//     //                     assert_eq!(direct[0], DirectRelation::Type("user".to_string()));
//     //                 }
//     //                 _ => panic!("Expected direct relation variant"),
//     //             }
//     //             match &or_relations[1] {
//     //                 RelationVariant::Conditional(inner_cond) => {
//     //                     assert_eq!(inner_cond.from.as_ref().unwrap(), "owner");
//     //                 }
//     //                 _ => panic!("Expected conditional relation variant"),
//     //             }
//     //         }
//     //         _ => panic!("Expected conditional relation"),
//     //     }
//     // }
//
//     #[test]
//     fn test_invalid_direct_relation() {
//         let yaml_data = r#"
//             - direct:
//                 - invalid_field: "user"
//         "#;
//
//         let result: Result<Vec<HashMap<String, Vec<DirectRelation>>>, _> = from_str(yaml_data);
//         assert!(result.is_err());
//     }
//
//     #[test]
//     fn test_convert_to_json_schema() {
//         setup();
//         let yaml_data = r#"
//             model:
//               schema: 1.1
//
//             types:
//               user:
//                 # No relations needed for user as it's a base type
//
//               domain:
//                 relations:
//                   member:
//                     direct: [user]  # Directly relate users as members of a domain
//         "#;
//
//         let config: dsl::Configuration = parse_dsl(yaml_data);
//         let json_schema = convert_to_json_schema(config);
//
//         let expected_json = json!({
//             "schema_version": "1.1",
//             "type_definitions": [
//                 {
//                     "type": "user"
//                 },
//                 {
//                     "type": "domain",
//                     "relations": {
//                         "member": {
//                             "this": {}
//                         }
//                     },
//                     "metadata": {
//                         "relations": {
//                             "member": {
//                                 "directly_related_user_types": [
//                                     {
//                                         "type": "user"
//                                     }
//                                 ]
//                             }
//                         }
//                     }
//                 }
//             ]
//         });
//
//         let mut actual_json = serde_json::to_value(&json_schema).unwrap();
//         let mut expected_json_clone = expected_json.clone();
//         sort_json_value(&mut actual_json["type_definitions"]);
//         sort_json_value(&mut expected_json_clone["type_definitions"]);
//
//         assert_eq!(actual_json, expected_json_clone);
//     }
//
//     #[test]
//     fn test_convert_to_json_schema_advanced() {
//         setup();
//         let yaml_data = r#"
//             model:
//               schema: 1.1
//
//             types:
//               user:
//                 # No relations needed for user as it's a base type
//
//               domain:
//                 relations:
//                   member:
//                     direct: [user]  # Directly relate users as members of a domain
//               folder:
//                 relations:
//                   can_share:
//                     direct: [user]  # Make sure to have user as direct relation here
//                   owner:
//                     or:
//                       - direct: [user, domain#member]
//                       - from: parent_folder
//                   parent_folder:
//                     direct: [folder]
//                   viewer:
//                     or:
//                       - direct: [user, domain#member]
//                       - writer
//                       - from: parent_folder
//                   writer:
//                     or:
//                       - direct: [user, domain#member]
//                       - owner
//                       - from: parent_folder
//               document:
//                 relations:
//                   can_share:
//                     direct: [writer]
//                   owner:
//                     or:
//                       - direct: [user, domain#member]
//                       - from: parent_folder
//                   parent_folder:
//                     direct: [folder]
//                   viewer:
//                     or:
//                       - direct: [user, domain#member]
//                       - writer
//                       - from: parent_folder
//                   writer:
//                     or:
//                       - direct: [user, domain#member]
//                       - owner
//                       - from: parent_folder
//         "#;
//
//         let config: dsl::Configuration = parse_dsl(yaml_data);
//         let json_schema = convert_to_json_schema(config);
//
//         let expected_json = json!({
//             "schema_version": "1.1",
//             "type_definitions": [
//                 {
//                     "type": "user"
//                 },
//                 {
//                     "type": "domain",
//                     "relations": {
//                         "member": {
//                             "this": {}
//                         }
//                     },
//                     "metadata": {
//                         "relations": {
//                             "member": {
//                                 "directly_related_user_types": [
//                                     {
//                                         "type": "user"
//                                     }
//                                 ]
//                             }
//                         }
//                     }
//                 },
//                 {
//                     "type": "folder",
//                     "relations": {
//                         "can_share": {
//                             "computedUserset": {
//                                 "object": "",
//                                 "relation": "writer"
//                             }
//                         },
//                         "owner": {
//                             "union": {
//                                 "child": [
//                                     {
//                                         "this": {}
//                                     },
//                                     {
//                                         "tupleToUserset": {
//                                             "tupleset": {
//                                                 "object": "",
//                                                 "relation": "parent_folder"
//                                             },
//                                             "computedUserset": {
//                                                 "object": "",
//                                                 "relation": "owner"
//                                             }
//                                         }
//                                     }
//                                 ]
//                             }
//                         },
//                         "parent_folder": {
//                             "this": {}
//                         },
//                         "viewer": {
//                             "union": {
//                                 "child": [
//                                     {
//                                         "this": {}
//                                     },
//                                     {
//                                         "computedUserset": {
//                                             "object": "",
//                                             "relation": "writer"
//                                         }
//                                     },
//                                     {
//                                         "tupleToUserset": {
//                                             "tupleset": {
//                                                 "object": "",
//                                                 "relation": "parent_folder"
//                                             },
//                                             "computedUserset": {
//                                                 "object": "",
//                                                 "relation": "viewer"
//                                             }
//                                         }
//                                     }
//                                 ]
//                             }
//                         },
//                         "writer": {
//                             "union": {
//                                 "child": [
//                                     {
//                                         "this": {}
//                                     },
//                                     {
//                                         "computedUserset": {
//                                             "object": "",
//                                             "relation": "owner"
//                                         }
//                                     },
//                                     {
//                                         "tupleToUserset": {
//                                             "tupleset": {
//                                                 "object": "",
//                                                 "relation": "parent_folder"
//                                             },
//                                             "computedUserset": {
//                                                 "object": "",
//                                                 "relation": "writer"
//                                             }
//                                         }
//                                     }
//                                 ]
//                             }
//                         }
//                     },
//                     "metadata": {
//                         "relations": {
//                             "owner": {
//                                 "directly_related_user_types": [
//                                     {
//                                         "type": "user"
//                                     },
//                                     {
//                                         "type": "domain",
//                                         "relation": "member"
//                                     }
//                                 ]
//                             },
//                             "parent_folder": {
//                                 "directly_related_user_types": [
//                                     {
//                                         "type": "folder"
//                                     }
//                                 ]
//                             },
//                             "viewer": {
//                                 "directly_related_user_types": [
//                                     {
//                                         "type": "user"
//                                     },
//                                     {
//                                         "type": "domain",
//                                         "relation": "member"
//                                     }
//                                 ]
//                             },
//                             "writer": {
//                                 "directly_related_user_types": [
//                                     {
//                                         "type": "user"
//                                     },
//                                     {
//                                         "type": "domain",
//                                         "relation": "member"
//                                     }
//                                 ]
//                             }
//                         }
//                     }
//                 },
//                 {
//                     "type": "document",
//                     "relations": {
//                         "can_share": {
//                             "computedUserset": {
//                                 "object": "",
//                                 "relation": "writer"
//                             }
//                         },
//                         "owner": {
//                             "union": {
//                                 "child": [
//                                     {
//                                         "this": {}
//                                     },
//                                     {
//                                         "tupleToUserset": {
//                                             "tupleset": {
//                                                 "object": "",
//                                                 "relation": "parent_folder"
//                                             },
//                                             "computedUserset": {
//                                                 "object": "",
//                                                 "relation": "owner"
//                                             }
//                                         }
//                                     }
//                                 ]
//                             }
//                         },
//                         "parent_folder": {
//                             "this": {}
//                         },
//                         "viewer": {
//                             "union": {
//                                 "child": [
//                                     {
//                                         "this": {}
//                                     },
//                                     {
//                                         "computedUserset": {
//                                             "object": "",
//                                             "relation": "writer"
//                                         }
//                                     },
//                                     {
//                                         "tupleToUserset": {
//                                             "tupleset": {
//                                                 "object": "",
//                                                 "relation": "parent_folder"
//                                             },
//                                             "computedUserset": {
//                                                 "object": "",
//                                                 "relation": "viewer"
//                                             }
//                                         }
//                                     }
//                                 ]
//                             }
//                         },
//                         "writer": {
//                             "union": {
//                                 "child": [
//                                     {
//                                         "this": {}
//                                     },
//                                     {
//                                         "computedUserset": {
//                                             "object": "",
//                                             "relation": "owner"
//                                         }
//                                     },
//                                     {
//                                         "tupleToUserset": {
//                                             "tupleset": {
//                                                 "object": "",
//                                                 "relation": "parent_folder"
//                                             },
//                                             "computedUserset": {
//                                                 "object": "",
//                                                 "relation": "writer"
//                                             }
//                                         }
//                                     }
//                                 ]
//                             }
//                         }
//                     },
//                     "metadata": {
//                         "relations": {
//                             "owner": {
//                                 "directly_related_user_types": [
//                                     {
//                                         "type": "user"
//                                     },
//                                     {
//                                         "type": "domain",
//                                         "relation": "member"
//                                     }
//                                 ]
//                             },
//                             "parent_folder": {
//                                 "directly_related_user_types": [
//                                     {
//                                         "type": "folder"
//                                     }
//                                 ]
//                             },
//                             "viewer": {
//                                 "directly_related_user_types": [
//                                     {
//                                         "type": "user"
//                                     },
//                                     {
//                                         "type": "domain",
//                                         "relation": "member"
//                                     }
//                                 ]
//                             },
//                             "writer": {
//                                 "directly_related_user_types": [
//                                     {
//                                         "type": "user"
//                                     },
//                                     {
//                                         "type": "domain",
//                                         "relation": "member"
//                                     }
//                                 ]
//                             }
//                         }
//                     }
//                 }
//             ]
//         });
//
//         let mut actual_json = serde_json::to_value(&json_schema).unwrap();
//         let mut expected_json_clone = expected_json.clone();
//         sort_json_value(&mut actual_json["type_definitions"]);
//         sort_json_value(&mut expected_json_clone["type_definitions"]);
//
//         assert_eq!(actual_json, expected_json_clone);
//     }
// }
