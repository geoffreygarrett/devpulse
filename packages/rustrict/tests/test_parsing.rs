#[cfg(test)]
mod tests {
    use super::*;
    use rustrict::graph::{Entity, RelationTuple};
    use rustrict::parse_relation_tuple;

    #[test]
    fn test_parse_relation_tuple_basic() {
        let input = "file:foo.pdf#owner@user:alice";
        let expected = RelationTuple {
            object: Entity::new("file", "foo.pdf"),
            relation: "owner".to_string(),
            user: Entity::new("user", "alice"),
        };

        let result = parse_relation_tuple(input);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_relation_tuple_group() {
        let input = "group:eng#member@user:11";
        let expected = RelationTuple {
            object: Entity::new("group", "eng"),
            relation: "member".to_string(),
            user: Entity::new("user", "11"),
        };

        let result = parse_relation_tuple(input);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_relation_tuple_nested() {
        let input = "doc:readme#viewer@group:eng#member";
        let expected = RelationTuple {
            object: Entity::new("doc", "readme"),
            relation: "viewer".to_string(),
            user: Entity::new("group", "eng#member"),
        };

        let result = parse_relation_tuple(input);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_relation_tuple_parent() {
        let input = "doc:readme#parent@folder:A#...";
        let expected = RelationTuple {
            object: Entity::new("doc", "readme"),
            relation: "parent".to_string(),
            user: Entity::new("folder", "A#..."),
        };

        let result = parse_relation_tuple(input);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_relation_tuple_invalid() {
        let input = "invalid:input";
        let result = ZanzibarParser::parse(Rule::tuple, input);
        assert!(result.is_err());
    }
}
