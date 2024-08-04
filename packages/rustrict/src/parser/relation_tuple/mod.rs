use std::convert::TryFrom;
use std::fmt;
use std::fmt::Display;
use std::str::FromStr;

use pest::iterators::{Pair, Pairs};
use pest::Parser;
use pest_derive::Parser;

#[derive(Debug)]
pub enum ParseError {
    Pest(pest::error::Error<Rule>),
    MissingPermission,
    MissingUser,
    MissingField(&'static str),
}

impl Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::Pest(e) => write!(f, "Parse error: {}", e),
            ParseError::MissingPermission => write!(f, "Missing permission field in the tuple"),
            ParseError::MissingUser => write!(f, "Missing user field in the tuple"),
            ParseError::MissingField(field) => write!(f, "Missing field: {}", field),
        }
    }
}

impl std::error::Error for ParseError {}

impl From<pest::error::Error<Rule>> for ParseError {
    fn from(error: pest::error::Error<Rule>) -> Self {
        ParseError::Pest(error)
    }
}

macro_rules! unexpected_parser_syntax {
    ($pair:expr) => {
        unimplemented!("unexpected parser rule: {:#?}\n\n {:#?}", $pair.as_rule(), $pair);
    };
}

#[derive(Parser)]
#[grammar = "grammar/relation_tuple.pest"]
pub struct ZanzibarParser;

#[derive(Debug, PartialEq)]
pub struct Object {
    pub namespace: String,
    pub id: String,
}

#[derive(Debug, PartialEq)]
pub struct Userset {
    pub object: Object,
    pub relation: String,
}

#[derive(Debug, PartialEq)]
pub enum Subject {
    User(String),
    Userset(Userset),
}

#[derive(Debug, PartialEq)]
pub struct Relationship {
    pub object: Object,
    pub relation: String,
    pub subject: Subject,
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
            .ok_or(ParseError::MissingField("object_id"))?
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
            _ => unexpected_parser_syntax!(pair),
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
            .ok_or(ParseError::MissingField("relation"))?
            .as_str()
            .to_string();
        let subject = Subject::try_from(pairs.next().ok_or(ParseError::MissingField("subject"))?)?;
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
            Subject::Userset(userset) => write!(f, "{}", userset),
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

pub fn parse_userset(input: &str) -> Result<Userset, ParseError> {
    ZanzibarParser::parse(Rule::userset, input)?
        .next()
        .ok_or(ParseError::MissingField("userset"))
        .and_then(Userset::try_from)
}

pub fn parse_subject(input: &str) -> Result<Subject, ParseError> {
    ZanzibarParser::parse(Rule::subject, input)?
        .next()
        .ok_or(ParseError::MissingField("subject"))
        .and_then(Subject::try_from)
}

pub fn parse_relationship(input: &str) -> Result<Relationship, ParseError> {
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
    fn test_parse_userset() {
        let input = "users:eng#member";
        let userset = parse_userset(input).unwrap();
        assert_eq!(userset.object.namespace, "users");
        assert_eq!(userset.object.id, "eng");
        assert_eq!(userset.relation, "member");
    }

    #[test]
    fn test_parse_subject_user() {
        let input = "10";
        let subject = parse_subject(input).unwrap();
        assert_eq!(subject, Subject::User("10".to_string()));
    }

    #[test]
    fn test_parse_subject_userset() {
        let input = "users:eng#member";
        let subject = parse_subject(input).unwrap();
        assert_eq!(
            subject,
            Subject::Userset(Userset {
                object: Object {
                    namespace: "users".to_string(),
                    id: "eng".to_string(),
                },
                relation: "member".to_string(),
            })
        );
    }

    #[test]
    fn test_parse_relationship() {
        let input = "doc:readme#owner@10";
        let tuple = parse_relationship(input).unwrap();
        assert_eq!(tuple.object.namespace, "doc");
        assert_eq!(tuple.object.id, "readme");
        assert_eq!(tuple.relation, "owner");
        assert_eq!(tuple.subject, Subject::User("10".to_string()));
    }

    #[test]
    fn test_parse_relationship_with_userset() {
        let input = "doc:readme#viewer@users:eng#member";
        let tuple = parse_relationship(input).unwrap();
        assert_eq!(tuple.object.namespace, "doc");
        assert_eq!(tuple.object.id, "readme");
        assert_eq!(tuple.relation, "viewer");
        assert_eq!(
            tuple.subject,
            Subject::Userset(Userset {
                object: Object {
                    namespace: "users".to_string(),
                    id: "eng".to_string(),
                },
                relation: "member".to_string(),
            })
        );
    }

    #[test]
    #[should_panic(expected = "Parse error: Error { location: 1:1, kind: Custom { message: \"unexpected parser rule: relationship\" } }")]
    fn test_parse_error() {
        let input = "invalid:input#format@10";
        parse_relationship(input).unwrap();
    }
}
