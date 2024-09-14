// use std::error::Error;
// use std::fs;
//
// use pest::iterators::Pairs;
// use pest::Parser;
// use pest_derive::Parser;
// use serde_json::{json, Value};
//
// #[derive(Parser)]
// #[grammar = "grammar/zanzibar.pest"]
// pub struct ConfigParser;
//
// fn parse_value(pair: pest::iterators::Pair<Rule>) -> Result<Value, Box<dyn Error>> {
//     match pair.as_rule() {
//         Rule::string => Ok(json!(pair.into_inner().next().unwrap().as_str())),
//         Rule::variable => Ok(json!("variable_object")),
//         _ => Err("Unsupported value type".into()),
//     }
// }
//
// fn parse_pair(pair: pest::iterators::Pair<Rule>) -> Result<(String, Value), Box<dyn Error>> {
//     let mut inner_rules = pair.into_inner();
//     let key = inner_rules.next().unwrap().as_str().to_string();
//     let value = parse_value(inner_rules.next().unwrap())?;
//     Ok((key, value))
// }
//
// fn parse_block(pair: pest::iterators::Pair<Rule>) -> Result<Value, Box<dyn Error>> {
//     let mut map = serde_json::Map::new();
//     for inner_pair in pair.into_inner() {
//         match inner_pair.as_rule() {
//             Rule::pair => {
//                 let (key, value) = parse_pair(inner_pair)?;
//                 map.insert(key, value);
//             }
//             Rule::block => {
//                 let block_value = parse_block(inner_pair.clone())?;
//                 let key = inner_pair
//                     .as_str()
//                     .split("{")
//                     .next()
//                     .unwrap()
//                     .trim()
//                     .to_string();
//                 map.insert(key, block_value);
//             }
//             _ => {}
//         }
//     }
//     Ok(Value::Object(map))
// }
//
// fn parse_computed_userset(pair: pest::iterators::Pair<Rule>) -> Result<Value, Box<dyn Error>> {
//     let mut computed_userset = serde_json::Map::new();
//     for inner_pair in pair.into_inner() {
//         let (key, value) = parse_pair(inner_pair)?;
//         computed_userset.insert(key, value);
//     }
//     Ok(Value::Object(computed_userset))
// }
//
// fn parse_relation(pair: pest::iterators::Pair<Rule>) -> Result<Value, Box<dyn Error>> {
//     let mut relation = serde_json::Map::new();
//     for inner_pair in pair.into_inner() {
//         match inner_pair.as_rule() {
//             Rule::name_pair => {
//                 let (key, value) = parse_pair(inner_pair)?;
//                 relation.insert(key, value);
//             }
//             Rule::userset_rewrite_block => {
//                 let userset_rewrite = parse_userset_rewrite(inner_pair)?;
//                 relation.insert("userset_rewrite".to_string(), userset_rewrite);
//             }
//             _ => {}
//         }
//     }
//     Ok(Value::Object(relation))
// }
//
// fn parse_userset_rewrite(pair: pest::iterators::Pair<Rule>) -> Result<Value, Box<dyn Error>> {
//     let mut array = Vec::new();
//     for inner_pair in pair.into_inner() {
//         if let Rule::union_block = inner_pair.as_rule() {
//             let union = parse_union(inner_pair)?;
//             array.push(union);
//         }
//     }
//     Ok(json!(array))
// }
//
// fn parse_union(pair: pest::iterators::Pair<Rule>) -> Result<Value, Box<dyn Error>> {
//     let mut array = Vec::new();
//     for inner_pair in pair.into_inner() {
//         let child = parse_child(inner_pair)?;
//         array.push(child);
//     }
//     Ok(json!(array))
// }
//
// fn parse_child(pair: pest::iterators::Pair<Rule>) -> Result<Value, Box<dyn Error>> {
//     let mut map = serde_json::Map::new();
//     for inner_pair in pair.into_inner() {
//         let insert_result = match inner_pair.as_rule() {
//             Rule::_this_block => map.insert("_this".to_string(), json!({})),
//             Rule::computed_userset => {
//                 let computed_userset = parse_computed_userset(inner_pair)?;
//                 map.insert("computed_userset".to_string(), computed_userset)
//             }
//             Rule::tuple_to_userset => {
//                 let tuple_to_userset = parse_tuple_to_userset(inner_pair)?;
//                 map.insert("tuple_to_userset".to_string(), tuple_to_userset)
//             }
//             _ => continue, // Skip any unrecognized rules or handle differently if needed
//         };
//
//         // Optionally check the result to handle overwrite cases or debug
//         if insert_result.is_some() {
//             // Log or handle the case where an existing key was overwritten, if necessary
//             eprintln!("Warning: Overwriting a value in the map.");
//         }
//     }
//     Ok(Value::Object(map))
// }
//
// fn parse_tuple_to_userset(pair: pest::iterators::Pair<Rule>) -> Result<Value, Box<dyn Error>> {
//     let mut map = serde_json::Map::new();
//     for inner_pair in pair.into_inner() {
//         match inner_pair.as_rule() {
//             Rule::tupleset => {
//                 let tupleset_value = parse_tupleset(inner_pair)?;
//                 map.insert("tupleset".to_string(), tupleset_value);
//             },
//             Rule::computed_userset => {
//                 let computed_userset_value = parse_computed_userset(inner_pair)?;
//                 map.insert("computed_userset".to_string(), computed_userset_value);
//             },
//             _ => return Err("Unsupported rule found in tuple_to_userset".into()),
//         }
//     }
//     Ok(Value::Object(map))
// }
//
// fn parse_tupleset(pair: pest::iterators::Pair<Rule>) -> Result<Value, Box<dyn Error>> {
//     let mut map = serde_json::Map::new();
//     for inner_pair in pair.into_inner() {
//         let (key, value) = parse_pair(inner_pair)?;
//         map.insert(key, value);
//     }
//     Ok(Value::Object(map))
// }
//
// fn parse_config(pairs: Pairs<Rule>) -> Result<Value, Box<dyn Error>> {
//     let mut config = serde_json::Map::new();
//     for pair in pairs {
//         match pair.as_rule() {
//             Rule::relation_block => {
//                 let relation = parse_relation(pair)?;
//                 config
//                     .entry("relations")
//                     .or_insert_with(|| json!([]))
//                     .as_array_mut()
//                     .unwrap()
//                     .push(relation);
//             }
//             Rule::name_pair => {
//                 let (key, value) = parse_pair(pair)?;
//                 config.insert(key, value);
//             }
//             _ => {}
//         }
//     }
//     Ok(Value::Object(config))
// }
//
//
//
// fn main() -> Result<(), Box<dyn Error>> {
//     let unparsed_file = fs::read_to_string("path/to/config.file")?;
//     let file = ConfigParser::parse(Rule::config, &unparsed_file)?
//         .next()
//         .unwrap();
//     let parsed_config = parse_config(file.into_inner())?;
//     println!("{:#?}", parsed_config);
//     Ok(())
// }
//
// #[cfg(test)]
// mod tests {
//     use pest::Parser;
//
//     use super::*;
//
//     // Helper function to quickly parse a given rule from a string
//     fn parse_rule(rule: Rule, input: &str) -> Result<Pairs<Rule>, pest::error::Error<Rule>> {
//         ConfigParser::parse(rule, input)
//     }
//
//     #[test]
//     fn test_parse_tuple_to_userset() {
//         let input = r#"
//     tuple_to_userset {
//         tupleset { relation: "owner" }
//         computed_userset { relation: "member" }
//     }
//     "#;
//         let mut pairs = parse_rule(Rule::tuple_to_userset, input).unwrap();
//         let result = parse_tuple_to_userset(pairs.next().unwrap()).unwrap();
//         let expected = json!({
//         "tupleset": {"relation": "owner"},
//         "computed_userset": {"relation": "member"}
//     });
//         assert_eq!(result, expected);
//     }
//
//
//     #[test]
//     fn test_parse_value_string() {
//         let input = "\"example string\"";
//         let pairs = parse_rule(Rule::string, input).unwrap();
//         let result = parse_value(pairs.peek().unwrap()).unwrap();
//         assert_eq!(result, json!("example string"));
//     }
//
//     #[test]
//     fn test_parse_value_variable() {
//         let input = "$TUPLE_USERSET_OBJECT";
//         let pairs = parse_rule(Rule::variable, input).unwrap();
//         let result = parse_value(pairs.peek().unwrap()).unwrap();
//         assert_eq!(result, json!("variable_object"));
//     }
//
//     #[test]
//     fn test_parse_pair() {
//         let input = "key:\"value\"";
//         let pairs = parse_rule(Rule::pair, input).unwrap();
//         let result = parse_pair(pairs.peek().unwrap()).unwrap();
//         assert_eq!(result, ("key".to_string(), json!("value")));
//     }
//
//     #[test]
//     fn test_parse_block_simple() {
//         let input = "config { key:\"value\" }";
//         let pairs = parse_rule(Rule::block, input).unwrap();
//         let result = parse_block(pairs.peek().unwrap()).unwrap();
//         assert_eq!(result, json!({"key": "value"}));
//     }
//
//     #[test]
//     fn test_parse_computed_userset() {
//         let input = "computed_userset { relation:\"owner\" object:\"$TUPLE_USERSET_OBJECT\" }";
//         let pairs = parse_rule(Rule::computed_userset, input).unwrap();
//         let result = parse_computed_userset(pairs.peek().unwrap()).unwrap();
//         assert_eq!(result, json!({"relation": "owner", "object": "variable_object"}));
//     }
//
//     #[test]
//     fn test_parse_relation() {
//         let input = "relation { name:\"admin\" userset_rewrite { union { child { _this {} } } } }";
//         let pairs = parse_rule(Rule::relation_block, input).unwrap();
//         let result = parse_relation(pairs.peek().unwrap()).unwrap();
//         let expected = json!({
//             "name": "admin",
//             "userset_rewrite": [
//                 {
//                     "union": [
//                         {
//                             "_this": {}
//                         }
//                     ]
//                 }
//             ]
//         });
//         assert_eq!(result, expected);
//     }
//
//     #[test]
//     fn test_parse_userset_rewrite() {
//         let input = "userset_rewrite { union { child { _this {} } child { computed_userset { relation:\"owner\" } } } }";
//         let pairs = parse_rule(Rule::userset_rewrite_block, input).unwrap();
//         let result = parse_userset_rewrite(pairs.peek().unwrap()).unwrap();
//         let expected = json!([
//             {
//                 "union": [
//                     {"_this": {}},
//                     {"computed_userset": {"relation": "owner"}}
//                 ]
//             }
//         ]);
//         assert_eq!(result, expected);
//     }
// }
