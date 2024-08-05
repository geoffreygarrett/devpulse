use std::collections::HashMap;
use std::error::Error;

use color_eyre::eyre::Result;
use pest::{
    iterators::Pair,
    Parser,
};
use pest_derive::Parser;
use serde::{Deserialize, Serialize};

mod error;
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
struct DirectUserset {}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct Wildcard {}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct ObjectRelation {
    #[serde(rename = "object", skip_serializing_if = "Option::is_none")]
    pub object: Option<String>,
    #[serde(rename = "relation")]
    pub relation: Option<String>,
}

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

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct TupleToUserset {
    #[serde(rename = "tupleset")]
    pub tupleset: ObjectRelation,
    #[serde(rename = "computedUserset")]
    pub computed_userset: ObjectRelation,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
enum Child {
    #[serde(rename = "direct_userset")]
    DirectUserset(DirectUserset),
    #[serde(rename = "tuple_to_userset")]
    TupleToUserset(TupleToUserset),
    #[serde(rename = "computed_userset")]
    ComputedUserset(ObjectRelation),
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
struct Children {
    child: Vec<Child>,
}

// #[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
// struct UsersetRewrite {
//     union: Option<Children>,
//     intersection: Option<Children>,
//     exclusion: Option<Children>,
// }

// #[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
// struct Relation {
//     name: String,
//     userset_rewrite: Option<UsersetRewrite>,
// }

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
struct Schema {
    version: String,
}
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
struct ModelConfig {
    schema: Schema,
    types: Vec<TypeDefinition>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
struct SourceInfo {
    file: String,
    line: u32,
    line_end: u32,
    column: u32,
    column_end: u32,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
struct UserType {
    #[serde(rename = "type")]
    _type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    relation: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct RelationMetadata {
    #[serde(rename = "directly_related_user_types")]
    pub directly_related_user_types: Option<Vec<RelationReference>>,
    #[serde(rename = "module")]
    pub module: Option<String>,
    #[serde(rename = "source_info")]
    pub source_info: Option<SourceInfo>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Metadata {
    #[serde(rename = "relations")]
    pub relations: HashMap<String, RelationMetadata>,
    #[serde(rename = "module")]
    pub module: Option<String>,
    #[serde(rename = "source_info")]
    pub source_info: Option<SourceInfo>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
struct TypeDefinition {
    #[serde(rename = "type")]
    pub _type: String,
    relations: Option<HashMap<String, UsersetRewrite>>,
    metadata: Option<Metadata>,
}

macro_rules! unexpected_parser_syntax {
    ($pair:expr) => {
        unimplemented!(
            "unexpected parser rule: {:#?}\n\n {:#?}",
            $pair.as_rule(),
            $pair
        );
    };
}

macro_rules! missing_field_error {
    ($field:expr) => {
        unimplemented!("missing field: {}", $field);
    };
}

#[derive(Parser)]
#[grammar = "grammar/fga.pest"]
struct FgaParser;

fn query_array_from_pair(pair: Pair<Rule>) -> Vec<String> {
    pair.into_inner()
        .map(|p| p.into_inner().next().unwrap().as_str().to_string())
        .collect()
}

fn query_string_from_pair(pair: Pair<Rule>) -> String {
    pair.as_str().to_string()
}

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

fn parse_identifier(pair: Pair<Rule>) -> String {
    query_string_from_pair(pair)
}

fn parse_this(pair: Pair<Rule>) -> Vec<RelationReference> {
    let mut direct_types: Vec<RelationReference> = vec![];
    for inner_pair in pair.into_inner() {
        match inner_pair.as_rule() {
            Rule::reference => {
                direct_types.push(parse_relation_reference(inner_pair));
            }
            _ => unexpected_parser_syntax!(inner_pair),
        }
    }
    direct_types
}

fn parse_computed_userset(pair: Pair<Rule>) -> ObjectRelation {
    let mut object = None;
    let mut relation = None;
    for inner_pair in pair.into_inner() {
        match inner_pair.as_rule() {
            Rule::object => {
                object = Some(query_string_from_pair(inner_pair));
            }
            Rule::relation => {
                relation = Some(query_string_from_pair(inner_pair));
            }
            _ => unexpected_parser_syntax!(inner_pair),
        }
    }
    ObjectRelation { object, relation }
}

fn parse_tuple_to_userset(pair: Pair<Rule>) -> TupleToUserset {
    let mut tupleset = None;
    let mut computed_userset = None;
    for inner_pair in pair.into_inner() {
        match inner_pair.as_rule() {
            Rule::tupleset => {
                tupleset = Some(ObjectRelation {
                    object: None,
                    relation: Some(query_string_from_pair(inner_pair)),
                });
            }
            Rule::computed_userset => {
                computed_userset = Some(ObjectRelation {
                    object: Some("$TUPLE_USERSET_OBJECT".to_string()),
                    relation: Some(query_string_from_pair(inner_pair)),
                });
            }
            _ => unexpected_parser_syntax!(inner_pair),
        }
    }
    TupleToUserset {
        tupleset: tupleset.unwrap(),
        computed_userset: computed_userset.unwrap(),
    }
}

fn parse_userset_rewrite(pair: Pair<Rule>) -> (UsersetRewrite, Vec<RelationReference>) {
    let mut this = None;
    let mut computed_userset = None;
    let mut tuple_to_userset = None;
    let mut union = None;
    let mut intersection = None;
    let mut exclusion = None;
    let mut references = Vec::new();

    for mut inner_pair in pair.into_inner() {
        if inner_pair.as_rule() == Rule::child {
            inner_pair = inner_pair.into_inner().next().unwrap();
        }
        match inner_pair.as_rule() {
            Rule::this => {
                this = Some(DirectUserset {});
                references = parse_this(inner_pair);
            }
            Rule::computed_userset => {
                computed_userset = Some(parse_computed_userset(inner_pair));
            }
            Rule::tuple_to_userset => {
                tuple_to_userset = Some(parse_tuple_to_userset(inner_pair));
            }
            Rule::union => {
                let (parsed_union, mut child_references): (Vec<_>, Vec<_>) = inner_pair
                    .into_inner()
                    .map(|p| parse_userset_rewrite(p))
                    .unzip();
                union = Some(parsed_union.into_iter().map(Box::new).collect());
                references.append(&mut child_references.concat());
            }
            Rule::intersection => {
                let (parsed_intersection, mut child_references): (Vec<_>, Vec<_>) = inner_pair
                    .into_inner()
                    .map(|p| parse_userset_rewrite(p))
                    .unzip();
                intersection = Some(parsed_intersection.into_iter().map(Box::new).collect());
                references.append(&mut child_references.concat());
            }
            Rule::exclusion => {
                let (parsed_exclusion, mut child_references): (Vec<_>, Vec<_>) = inner_pair
                    .into_inner()
                    .map(|p| parse_userset_rewrite(p))
                    .unzip();
                exclusion = Some(parsed_exclusion.into_iter().map(Box::new).collect());
                references.append(&mut child_references.concat());
            }
            _ => unexpected_parser_syntax!(inner_pair),
        }
    }

    (
        UsersetRewrite {
            this,
            computed_userset,
            tuple_to_userset,
            union,
            intersection,
            exclusion,
        },
        references,
    )
}


fn parse_relation_definition(pair: Pair<Rule>) -> (String, RelationMetadata, UsersetRewrite) {
    let mut name = None;
    let mut userset_rewrite = None;
    for inner_pair in pair.clone().into_inner() {
        match inner_pair.as_rule() {
            Rule::relation => {
                name = Some(parse_identifier(inner_pair));
            }
            Rule::userset_rewrite => {
                userset_rewrite = Some(parse_userset_rewrite(inner_pair));
            }
            _ => unexpected_parser_syntax!(inner_pair),
        }
    }
    let (userset_rewrite, references) = userset_rewrite.unwrap();

    if let Some(name) = name {
        let pair_span = pair.as_span();
        (name, RelationMetadata {
            directly_related_user_types: Some(references),
            module: None,
            source_info: Some(SourceInfo {
                file: "file".to_string(),
                line: pair_span.start_pos().line_col().0 as u32,
                line_end: pair_span.end_pos().line_col().0 as u32,
                column: pair_span.start_pos().line_col().1 as u32,
                column_end: pair_span.end_pos().line_col().1 as u32,
            }),
        },
         userset_rewrite)
    } else {
        panic!("Relation name is required");
    }
}


fn parse_relations(pair: Pair<Rule>) -> (HashMap<String, RelationMetadata>, HashMap<String, UsersetRewrite>) {
    let mut relations: HashMap<String, UsersetRewrite> = HashMap::new();
    let mut metadata: HashMap<String, RelationMetadata> = HashMap::new();
    for inner_pair in pair.into_inner() {
        let (name, direct, relation) = parse_relation_definition(inner_pair);
        println!("{:?}, {:?}", direct, relation);
        relations.insert(name.clone(), relation);
        metadata.insert(name, direct);
    }
    (metadata, relations)
}


fn parse_type_definition(pair: Pair<Rule>) -> TypeDefinition {
    let mut metadata: HashMap<String, RelationMetadata> = HashMap::new();
    let mut _type = String::new();
    let mut relations: HashMap<String, UsersetRewrite> = HashMap::new();
    for inner_pair in pair.clone().into_inner() {
        // Subtype definition
        // - identifier
        // - relations
        match inner_pair.as_rule() {
            Rule::identifier => {
                _type = query_string_from_pair(inner_pair);
            }
            Rule::relations => {
                let (direct, relation) = parse_relations(inner_pair);
                relations = relation;
                metadata = direct;
            }
            _ => unexpected_parser_syntax!(inner_pair),
        }
    }

    let span = pair.as_span();
    TypeDefinition {
        _type,
        relations: Some(relations),
        metadata: Option::from(Metadata {
            relations: metadata,
            module: None,
            source_info: Some(SourceInfo {
                file: "file".to_string(),
                line: span.start_pos().line_col().0 as u32,
                line_end: span.end_pos().line_col().0 as u32,
                column: span.start_pos().line_col().1 as u32,
                column_end: span.end_pos().line_col().1 as u32,
            }),
        }),
    }
}
fn parse_schema(pair: Pair<Rule>) -> Schema {
    let mut properties: HashMap<String, String> = HashMap::new();
    let mut version = String::new();
    for inner_pair in pair.into_inner() {
        match inner_pair.as_rule() {
            Rule::version => { version = query_string_from_pair(inner_pair); }
            _ => unexpected_parser_syntax!(inner_pair),
        }
    }
    Schema { version }
}

fn parse_model(pair: Pair<Rule>) -> ModelConfig {
    let mut schema: Schema;
    let mut types: Vec<TypeDefinition> = vec![];
    let mut module: Option<String> = None;

    for inner_pair in pair.into_inner() {
        match inner_pair.as_rule() {
            // Model definition
            Rule::schema => { schema = parse_schema(inner_pair) }
            Rule::module => { module = Some(query_string_from_pair(inner_pair.into_inner().next().unwrap())); }
            Rule::type_definition => { types.push(parse_type_definition(inner_pair)); }
            Rule::condition_definition => {}
            _ => unexpected_parser_syntax!(inner_pair),
        }
    }

    types.iter_mut().for_each(|t| {
        t.metadata.as_mut().unwrap().module = module.clone();
    });

    ModelConfig {
        schema: Schema { version: "1.1".to_string() },
        types,
    }
}

fn parse(file: &str) -> Result<ModelConfig, Box<dyn Error>> {
    let mut pairs = FgaParser::parse(Rule::input, file)?;
    let pair = pairs.next().unwrap();
    match pair.as_rule() {
        Rule::input => {
            Ok(parse_model(pair))
        }
        _ => unexpected_parser_syntax!(pair),
    }
}

fn parse_relation_reference(pair: Pair<Rule>) -> RelationReference {
    let mut _type_temp = None;
    let mut wildcard = None;
    let mut relation = None;
    let mut condition = None;

    for inner_pair in pair.clone().into_inner() {
        match inner_pair.as_rule() {
            Rule::_type => _type_temp = Some(query_string_from_pair(inner_pair)),
            Rule::relation => relation = Some(query_string_from_pair(inner_pair)),
            Rule::wildcard => wildcard = Some(Wildcard {}),
            Rule::condition => condition = Some(query_string_from_pair(inner_pair)),
            _ => unexpected_parser_syntax!(inner_pair),
        }
    }
    if let Some(_type) = _type_temp {
        RelationReference { _type, relation, wildcard, condition }
    } else {
        panic!("Type is required: {:?}", pair.as_str());
    }
}


#[cfg(test)]
mod tests {
    use pest::error::LineColLocation;

    use crate::parser::okta_dsl::error::display_error_lines;

    use super::*;

    #[test]
    pub fn test_parse_type_relation() {
        let input: Vec<&str> = vec!["team#member", "team#owner", "team#viewer", "team:*", "user with non_expired_grant"];
        let expected: Vec<RelationReference> = vec![
            RelationReference { _type: "team".to_string(), relation: Some("member".to_string()), wildcard: None, condition: None },
            RelationReference { _type: "team".to_string(), relation: Some("owner".to_string()), wildcard: None, condition: None },
            RelationReference { _type: "team".to_string(), relation: Some("viewer".to_string()), wildcard: None, condition: None },
            RelationReference { _type: "team".to_string(), relation: None, wildcard: Some(Wildcard {}), condition: None },
            RelationReference { _type: "user".to_string(), relation: None, wildcard: None, condition: Some("non_expired_grant".to_string()) },
        ];
        for (i, s) in input.iter().enumerate() {
            let mut result = FgaParser::parse(Rule::reference, s).unwrap();
            let result = parse_relation_reference(result.next().unwrap());
            assert_eq!(result, expected[i]);
        }
    }

    #[test]
    pub fn test_parse_this() {
        let input: &str = "[user, team#member, team#owner, team#viewer, team:*, user with non_expired_grant, team#viewer with non_expired_grant]";
        let expected: Vec<RelationReference> = vec![
            RelationReference { _type: "user".to_string(), relation: None, wildcard: None, condition: None },
            RelationReference { _type: "team".to_string(), relation: Some("member".to_string()), wildcard: None, condition: None },
            RelationReference { _type: "team".to_string(), relation: Some("owner".to_string()), wildcard: None, condition: None },
            RelationReference { _type: "team".to_string(), relation: Some("viewer".to_string()), wildcard: None, condition: None },
            RelationReference { _type: "team".to_string(), relation: None, wildcard: Some(Wildcard {}), condition: None },
            RelationReference { _type: "user".to_string(), relation: None, wildcard: None, condition: Some("non_expired_grant".to_string()) },
            RelationReference { _type: "team".to_string(), relation: Some("viewer".to_string()), wildcard: None, condition: Some("non_expired_grant".to_string()) },
        ];
        let mut parsed = FgaParser::parse(Rule::this, input).unwrap();
        let result = parse_this(parsed.next().unwrap());
        assert_eq!(result, expected);
    }

    #[test]
    pub fn test_parse_userset_base() {
        let input: &str = "[user]";
        let pair = FgaParser::parse(Rule::userset_rewrite, input).expect("Failed to parse input").next().unwrap();
        let result = parse_userset_rewrite(pair);
        let expected = UsersetRewrite {
            this: Some(DirectUserset {}),
            computed_userset: None,
            tuple_to_userset: None,
            union: None,
            intersection: None,
            exclusion: None,
        };
    }

    #[test]
    pub fn test_parse_userset_with_computed_userset() {
        let input: &str = "writer";
        let pair = FgaParser::parse(Rule::userset_rewrite, input).expect("Failed to parse input").next().unwrap();
        let result = parse_userset_rewrite(pair);
        let expected = UsersetRewrite {
            this: None,
            computed_userset: Some(ObjectRelation { object: None, relation: Some("writer".to_string()) }),
            tuple_to_userset: None,
            union: None,
            intersection: None,
            exclusion: None,
        };
    }

    #[test]
    pub fn test_parse_userset_with_tuple_to_userset() {
        let input: &str = "parent_folder#viewer";
        let pair = FgaParser::parse(Rule::userset_rewrite, input).expect("Failed to parse input").next().unwrap();
        let result = parse_userset_rewrite(pair);
        let expected = UsersetRewrite {
            this: None,
            computed_userset: None,
            tuple_to_userset: Some(TupleToUserset {
                tupleset: ObjectRelation {
                    object: None,
                    relation: Some("parent_folder".to_string()),
                },
                computed_userset: ObjectRelation {
                    object: Some("$TUPLE_USERSET_OBJECT".to_string()),
                    relation: Some("viewer".to_string()),
                },
            }),
            union: None,
            intersection: None,
            exclusion: None,
        };
    }

    #[test]
    pub fn test_parse_userset_rewrite_union() {
        let input: &str = r#"[user, domain#member] or writer or viewer from parent_folder"#;
        let pair = FgaParser::parse(Rule::userset_rewrite, input).expect("Failed to parse input").next().unwrap();
        let result = parse_userset_rewrite(pair);

        let expected = UsersetRewrite {
            this: None,
            computed_userset: None,
            tuple_to_userset: None,
            union: Some(vec![
                Box::new(UsersetRewrite {
                    this: Some(DirectUserset {}),
                    computed_userset: None,
                    tuple_to_userset: None,
                    union: None,
                    intersection: None,
                    exclusion: None,
                }),
                Box::new(UsersetRewrite {
                    this: None,
                    computed_userset: Some(ObjectRelation { object: None, relation: Some("writer".to_string()) }),
                    tuple_to_userset: None,
                    union: None,
                    intersection: None,
                    exclusion: None,
                }),
                Box::new(UsersetRewrite {
                    this: None,
                    computed_userset: None,
                    tuple_to_userset: Some(TupleToUserset {
                        tupleset: ObjectRelation {
                            object: None,
                            relation: Some("parent_folder".to_string()),
                        },
                        computed_userset: ObjectRelation {
                            object: Some("$TUPLE_USERSET_OBJECT".to_string()),
                            relation: Some("viewer".to_string()),
                        },
                    }),
                    union: None,
                    intersection: None,
                    exclusion: None,
                }),
            ]),
            intersection: None,
            exclusion: None,
        };

        assert_eq!(result.0, expected);
    }

    #[test]
    pub fn test_parse_userset_rewrite_intersection() {
        let input: &str = r#"[user, domain#member] and writer and viewer from parent_folder"#;
        let pair = FgaParser::parse(Rule::userset_rewrite, input).expect("Failed to parse input").next().unwrap();
        let result = parse_userset_rewrite(pair);

        let expected = UsersetRewrite {
            this: None,
            computed_userset: None,
            tuple_to_userset: None,
            union: None,
            intersection: Some(vec![
                Box::new(UsersetRewrite {
                    this: Some(DirectUserset {}),
                    computed_userset: None,
                    tuple_to_userset: None,
                    union: None,
                    intersection: None,
                    exclusion: None,
                }),
                Box::new(UsersetRewrite {
                    this: None,
                    computed_userset: Some(ObjectRelation { object: None, relation: Some("writer".to_string()) }),
                    tuple_to_userset: None,
                    union: None,
                    intersection: None,
                    exclusion: None,
                }),
                Box::new(UsersetRewrite {
                    this: None,
                    computed_userset: None,
                    tuple_to_userset: Some(TupleToUserset {
                        tupleset: ObjectRelation {
                            object: None,
                            relation: Some("parent_folder".to_string()),
                        },
                        computed_userset: ObjectRelation {
                            object: Some("$TUPLE_USERSET_OBJECT".to_string()),
                            relation: Some("viewer".to_string()),
                        },
                    }),
                    union: None,
                    intersection: None,
                    exclusion: None,
                }),
            ]),
            exclusion: None,
        };

        assert_eq!(result.0, expected);
    }

    #[test]
    pub fn test_parse_userset_rewrite_exclusion() {
        let input: &str = r#"[user, domain#member] but not writer but not viewer from parent_folder"#;
        let pair = FgaParser::parse(Rule::userset_rewrite, input).expect("Failed to parse input").next().unwrap();
        let result = parse_userset_rewrite(pair);

        let expected = UsersetRewrite {
            this: None,
            computed_userset: None,
            tuple_to_userset: None,
            union: None,
            intersection: None,
            exclusion: Some(vec![
                Box::new(UsersetRewrite {
                    this: Some(DirectUserset {}),
                    computed_userset: None,
                    tuple_to_userset: None,
                    union: None,
                    intersection: None,
                    exclusion: None,
                }),
                Box::new(UsersetRewrite {
                    this: None,
                    computed_userset: Some(ObjectRelation { object: None, relation: Some("writer".to_string()) }),
                    tuple_to_userset: None,
                    union: None,
                    intersection: None,
                    exclusion: None,
                }),
                Box::new(UsersetRewrite {
                    this: None,
                    computed_userset: None,
                    tuple_to_userset: Some(TupleToUserset {
                        tupleset: ObjectRelation {
                            object: None,
                            relation: Some("parent_folder".to_string()),
                        },
                        computed_userset: ObjectRelation {
                            object: Some("$TUPLE_USERSET_OBJECT".to_string()),
                            relation: Some("viewer".to_string()),
                        },
                    }),
                    union: None,
                    intersection: None,
                    exclusion: None,
                }),
            ]),
        };

        assert_eq!(result.0, expected);
    }


    /// Example function demonstrating usage
    pub fn parse_and_display_errors(input: &str) {
        let parsed = FgaParser::parse(Rule::model, input);
        match parsed {
            Ok(_) => println!("Parsing succeeded"),
            Err(err) => {
                eprintln!("{}", err);
                match err.line_col {
                    LineColLocation::Pos((line, column)) => {
                        eprintln!("{}", display_error_lines(input, line, column));
                    }
                    LineColLocation::Span((start_line, start_col), (end_line, end_col)) => {
                        eprintln!("Error from {}:{} to {}:{}", start_line, start_col, end_line, end_col);
                        eprintln!("{}", display_error_lines(input, start_line, start_col));
                    }
                }
            }
        }
    }

    #[test]
    fn test_parse_okta_dsl() {
        let input = r#"
            module example_
            model
              schema 1.1
            type user
            type domain
              relations define member: [user] or test
            type folder
              relations
                define viewer1: [user]
                define viewer2: [user] or x from y
                define viewer3: [user] or z
        "#;

        let result = parse(input).unwrap();
        // Check for the presence of specific types
        assert!(result.types.iter().any(|t| t._type == "user"));
        assert!(result.types.iter().any(|t| t._type == "domain"));
        assert!(result.types.iter().any(|t| t._type == "folder"));
        println!("{}", serde_json::to_string_pretty(&result).unwrap());
    }
}

// fn format(parsed: pest::iterators::Pair<Rule>) -> String {
//     let mut formatted_output = String::new();
//
//     for pair in parsed.into_inner() {
//         match pair.as_rule() {
//             Rule::model => {
//                 formatted_output.push_str("model\n");
//                 formatted_output.push_str(&format(pair));
//             }
//             Rule::schema => {
//                 formatted_output.push_str(pair.as_str());
//                 formatted_output.push('\n');
//             }
//             Rule::type_definition => {
//                 formatted_output.push_str(pair.as_str());
//                 formatted_output.push('\n');
//             }
//             Rule::relations => {
//                 formatted_output.push_str("        relati