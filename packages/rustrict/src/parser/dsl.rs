use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "../schema.pest"] // the grammar file name
pub struct SchemaParser;

pub fn parse_schema(input: &str) -> Result<Schema, String> {
    let pairs = SchemaParser::parse(Rule::start, input)
        .map_err(|e| e.to_string())?;

    let mut schema = Schema {
        schema_version: "1.1".to_string(),
        type_definitions: Vec::new(),
    };

    for pair in pairs {
        match pair.as_rule() {
            Rule::type_definition => {
                let type_def = parse_type_definition(pair);
                schema.type_definitions.push(type_def);
            },
            _ => {}
        }
    }

    Ok(schema)
}

fn parse_type_definition(pair: pest::iterators::Pair<Rule>) -> TypeDefinition {
    // Implementation to parse type definitions
    unimplemented!();
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_schema() {
        let input = r#"
            type User {
                relations: {
                    "owns": ["Photo"]
                }
            }
        "#;

        let schema = parse_schema(input).unwrap();
        assert_eq!(schema.schema_version, "1.1");
        assert_eq!(schema.type_definitions.len(), 1);
    }
}