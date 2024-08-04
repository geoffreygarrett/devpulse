use std::str::FromStr;

use pest::Parser;
use snafu::prelude::*;

use crate::errors::{MissingFieldSnafu, ProtobufError};

pub(crate) mod parser;

mod external {
    pub use crate::acl::*;
    pub use crate::acl::relation_tuple::*;
}

mod internal {
    use std::str::FromStr;

    use pest::Parser;

    use crate::errors::ParseError;
    use crate::models::acl::parser::{Rule, ZanzibarParser};

    // Define the structures using generic IDs
    #[derive(Debug, PartialEq, Clone, Eq, Hash)]
    pub struct Object {
        pub namespace: String,
        pub id: String,
    }

    #[derive(Debug, PartialEq, Clone, Eq, Hash)]
    pub struct Userset {
        pub object: Object,
        pub relation: String,
    }

    #[derive(Debug, PartialEq, Clone, Eq, Hash)]
    pub enum Subject {
        User(String),
        Userset(Userset),
    }

    #[derive(Debug, PartialEq, Clone, Eq, Hash)]
    pub struct Relationship {
        pub subject: Subject,
        pub relation: String,
        pub object: Object,
    }

    impl FromStr for Object {
        type Err = ParseError;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            Object::try_from(ZanzibarParser::parse(Rule::object, s)?.next().unwrap())
        }
    }

    impl FromStr for Userset {
        type Err = ParseError;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            Userset::try_from(ZanzibarParser::parse(Rule::userset, s)?.next().unwrap())
        }
    }

    impl FromStr for Subject {
        type Err = ParseError;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            Subject::try_from(ZanzibarParser::parse(Rule::subject, s)?.next().unwrap())
        }
    }

    impl FromStr for Relationship {
        type Err = ParseError;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            Relationship::try_from(
                ZanzibarParser::parse(Rule::relationship, s)?
                    .next()
                    .unwrap(),
            )
        }
    }
}

impl TryFrom<external::RelationTuple> for internal::Relationship {
    type Error = ProtobufError;

    fn try_from(relation: external::RelationTuple) -> Result<Self, Self::Error> {
        let subject = relation.subject.context(MissingFieldSnafu {
            field: "subject".to_string(),
        })?;

        let object = relation.object.context(MissingFieldSnafu {
            field: "object".to_string(),
        })?;

        Ok(Self {
            subject: subject.try_into()?,
            relation: relation.relation,
            object: object.into(),
        })
    }
}

impl From<external::Object> for internal::Object {
    fn from(object: external::Object) -> Self {
        Self {
            namespace: object.name,
            id: object.id,
        }
    }
}

impl TryFrom<external::Subject> for internal::Subject {
    type Error = ProtobufError;

    fn try_from(subject: external::Subject) -> Result<Self, Self::Error> {
        match subject {
            external::Subject::Id(user) => Ok(Self::User(user)),
            external::Subject::Userset(userset) => {
                let userset = userset.try_into()?;
                Ok(Self::Userset(userset))
            }
        }
    }
}

impl TryFrom<external::Userset> for internal::Userset {
    type Error = ProtobufError;

    fn try_from(userset: external::Userset) -> Result<Self, Self::Error> {
        let object = userset
            .object
            .context(MissingFieldSnafu {
                field: "object".to_string(),
            })?
            .into();

        Ok(Self {
            object,
            relation: userset.relation,
        })
    }
}

impl From<internal::Object> for external::Object {
    fn from(object: internal::Object) -> Self {
        Self {
            name: object.namespace,
            id: object.id,
        }
    }
}

impl From<internal::Userset> for external::Userset {
    fn from(userset: internal::Userset) -> Self {
        Self {
            object: Some(userset.object.into()),
            relation: userset.relation,
        }
    }
}

impl From<internal::Subject> for external::Subject {
    fn from(subject: internal::Subject) -> Self {
        match subject {
            internal::Subject::User(user) => Self::Id(user),
            internal::Subject::Userset(userset) => Self::Userset(userset.into()),
        }
    }
}

impl From<internal::Relationship> for external::RelationTuple {
    fn from(relationship: internal::Relationship) -> Self {
        let object = Some(relationship.object.into());
        let relation = relationship.relation;
        let subject = match relationship.subject {
            internal::Subject::User(user) => external::relation_tuple::Subject::Id(user),
            internal::Subject::Userset(userset) => {
                external::relation_tuple::Subject::Userset(userset.into())
            }
        };

        Self {
            object,
            relation,
            subject: Some(subject),
        }
    }
}
