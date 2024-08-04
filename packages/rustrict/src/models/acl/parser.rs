use std::convert::TryFrom;
use std::fmt;
use std::fmt::Display;
use std::str::FromStr;

use pest::iterators::Pair;
use pest::Parser;
use pest_derive::Parser;

use crate::errors::ParseError;
use crate::models::acl::internal::{Object, Relationship, Subject, Userset};

#[derive(Parser)]
#[grammar = "grammar/relation_tuple.pest"]
pub struct ZanzibarParser;

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::Pest(e) => write!(f, "Parse error: {}", e),
            ParseError::MissingPermission => write!(f, "Missing permission field in the tuple"),
            ParseError::MissingUser => write!(f, "Missing user field in the tuple"),
            ParseError::MissingField(field) => write!(f, "Missing field: {}", field),
        }
    }
}

impl From<pest::error::Error<Rule>> for ParseError {
    fn from(error: pest::error::Error<Rule>) -> Self {
        ParseError::Pest(error)
    }
}

impl TryFrom<Pair<'_, Rule>> for Object {
    type Error = ParseError;

    fn try_from(pair: Pair<Rule>) -> Result<Self, Self::Error> {
        let mut parts = pair.into_inner();
        let namespace = parts
            .next()
            .ok_or(ParseError::MissingField("namespace"))?
            .as_str()
            .to_string();
        let id = parts
            .next()
            .ok_or(ParseError::MissingField("id"))?
            .as_str()
            .to_string();
        Ok(Object { namespace, id })
    }
}

impl TryFrom<&str> for Object {
    type Error = ParseError;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let pair = ZanzibarParser::parse(Rule::object, s)?
            .next()
            .ok_or(ParseError::MissingField("object"))?;
        Object::try_from(pair)
    }
}

impl TryFrom<Pair<'_, Rule>> for Userset {
    type Error = ParseError;

    fn try_from(pair: Pair<Rule>) -> Result<Self, Self::Error> {
        let mut parts = pair.into_inner();
        let object_pair = parts.next().ok_or(ParseError::MissingField("object"))?;
        let object = Object::try_from(object_pair)?;
        let relation = parts
            .next()
            .ok_or(ParseError::MissingField("relation"))?
            .as_str()
            .to_string();
        Ok(Userset { object, relation })
    }
}

impl TryFrom<Pair<'_, Rule>> for Subject {
    type Error = ParseError;

    fn try_from(pair: Pair<Rule>) -> Result<Self, Self::Error> {
        match pair.as_rule() {
            Rule::user => Ok(Subject::User(pair.as_str().to_string())),
            Rule::userset => {
                let userset = Userset::try_from(pair)?;
                Ok(Subject::Userset(userset))
            }
            _ => unreachable!(),
        }
    }
}

impl TryFrom<Pair<'_, Rule>> for Relationship {
    type Error = ParseError;

    fn try_from(pair: Pair<Rule>) -> Result<Self, Self::Error> {
        let mut pairs = pair.into_inner();
        let object = Object::try_from(pairs.next().ok_or(ParseError::MissingField("object"))?)?;
        let relation = pairs
            .next()
            .ok_or(ParseError::MissingPermission)?
            .as_str()
            .to_string();
        let subject = Subject::try_from(pairs.next().ok_or(ParseError::MissingUser)?)?;
        Ok(Relationship {
            object,
            relation,
            subject,
        })
    }
}

impl Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.namespace, self.id)
    }
}

impl Display for Userset {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}#{}", self.object, self.relation)
    }
}

impl Display for Subject {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Subject::User(id) => write!(f, "user:{}", id),
            Subject::Userset(Userset) => write!(f, "{}", Userset),
        }
    }
}

impl Display for Relationship {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}#{}@{}", self.object, self.relation, self.subject)
    }
}

pub fn parse_object(input: &str) -> Result<Object, ParseError> {
    Object::try_from(input)
}

pub fn parse_group(input: &str) -> Result<Userset, ParseError> {
    ZanzibarParser::parse(Rule::userset, input)?
        .next()
        .ok_or(ParseError::MissingField("Userset"))
        .and_then(Userset::try_from)
}

pub fn parse_subject(input: &str) -> Result<Subject, ParseError> {
    ZanzibarParser::parse(Rule::subject, input)?
        .next()
        .ok_or(ParseError::MissingUser)
        .and_then(Subject::try_from)
}

pub fn parse_relation(input: &str) -> Result<Relationship, ParseError> {
    ZanzibarParser::parse(Rule::relationship, input)?
        .next()
        .ok_or(ParseError::MissingField("relationship"))
        .and_then(Relationship::try_from)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_object() {
        let input = "doc:readme";
        let object = parse_object(input).unwrap();
        assert_eq!(object.namespace, "doc");
        assert_eq!(object.id, "readme");
    }

    #[test]
    fn test_parse_group() {
        let input = "Userset:eng#member";
        let Userset = parse_group(input).unwrap();
        assert_eq!(Userset.object.namespace, "Userset");
        assert_eq!(Userset.object.id, "eng");
        assert_eq!(Userset.relation, "member");
    }

    #[test]
    fn test_parse_subject_user() {
        let input = "10";
        let subject = parse_subject(input).unwrap();
        assert_eq!(subject, Subject::User("10".to_string()));
    }

    #[test]
    fn test_parse_subject_group() {
        let input = "Userset:eng#member";
        let subject = parse_subject(input).unwrap();
        assert_eq!(
            subject,
            Subject::Userset(Userset {
                object: Object {
                    namespace: "Userset".to_string(),
                    id: "eng".to_string(),
                },
                relation: "member".to_string(),
            })
        );
    }

    #[test]
    fn test_parse_relation_tuple() {
        let input = "doc:readme#owner@10";
        let tuple = parse_relation(input).unwrap();
        assert_eq!(tuple.object.namespace, "doc");
        assert_eq!(tuple.object.id, "readme");
        assert_eq!(tuple.relation, "owner");
        assert_eq!(tuple.subject, Subject::User("10".to_string()));
    }

    #[test]
    fn test_parse_relation_tuple_with_group() {
        let input = "doc:readme#viewer@Userset:eng#member";
        let tuple = parse_relation(input).unwrap();
        assert_eq!(tuple.object.namespace, "doc");
        assert_eq!(tuple.object.id, "readme");
        assert_eq!(tuple.relation, "viewer");
        assert_eq!(
            tuple.subject,
            Subject::Userset(Userset {
                object: Object {
                    namespace: "Userset".to_string(),
                    id: "eng".to_string(),
                },
                relation: "member".to_string(),
            })
        );
    }

    #[test]
    fn test_parse_relation_tuple_with_period() {
        let input = "file:foo.pdf#owner@alice";
        let tuple = parse_relation(input).unwrap();
        assert_eq!(tuple.object.namespace, "file");
        assert_eq!(tuple.object.id, "foo.pdf");
        assert_eq!(tuple.relation, "owner");
        assert_eq!(tuple.subject, Subject::User("alice".to_string()));
    }
}
