use crate::config::models::{
    ConditionalRelation, Configuration, DirectRelation, IndirectRelation, Relation, RelationConfig,
    RelationVariant, TypeConfig,
};
use crate::config::okta_fga::json::models as okta_fga;

impl From<Configuration> for okta_fga::TypeDefinition {
    fn from(config: Configuration) -> Self {
        okta_fga::TypeDefinition {
            _type: config.model.schema,
            relations: config
                .types
                .into_iter()
                .map(|(k, v)| (k, v.into()))
                .collect(),
            metadata: None, // Assuming metadata is not provided in Configuration
        }
    }
}

impl From<TypeConfig> for okta_fga::Userset {
    fn from(config: TypeConfig) -> Self {
        okta_fga::Userset {
            this: None,             // Assuming "this" is not provided in TypeConfig
            computed_userset: None, // Assuming computed_userset is not provided in TypeConfig
            tuple_to_userset: None, // Assuming tuple_to_userset is not provided in TypeConfig
            union: config
                .relations
                .as_ref()
                .map(|relations| okta_fga::Usersets {
                    child: relations.values().cloned().map(|r| r.into()).collect(),
                }),
            intersection: None, // Assuming intersection is not provided in TypeConfig
            difference: None,   // Assuming difference is not provided in TypeConfig
        }
    }
}

impl From<RelationConfig> for okta_fga::Userset {
    fn from(config: RelationConfig) -> Self {
        match config {
            RelationConfig::Relation(relation) => relation.into(),
            RelationConfig::Conditional(conditional) => okta_fga::Userset {
                this: None,
                computed_userset: None,
                tuple_to_userset: None,
                union: conditional.union.map(|u| okta_fga::Usersets {
                    child: u.into_iter().map(|r| r.into()).collect(),
                }),
                intersection: conditional.intersection.map(|i| okta_fga::Usersets {
                    child: i.into_iter().map(|r| r.into()).collect(),
                }),
                difference: conditional.exclusion.map(|e| okta_fga::V1Difference {
                    base: Box::new(okta_fga::Userset {
                        this: None,
                        computed_userset: None,
                        tuple_to_userset: None,
                        union: Some(okta_fga::Usersets {
                            child: e.into_iter().map(|r| r.into()).collect(),
                        }),
                        intersection: None,
                        difference: None,
                    }),
                    subtract: Box::new(okta_fga::Userset {
                        this: None,
                        computed_userset: None,
                        tuple_to_userset: None,
                        union: None,
                        intersection: None,
                        difference: None,
                    }),
                }),
            },
        }
    }
}

impl From<Relation> for okta_fga::Userset {
    fn from(relation: Relation) -> Self {
        match relation {
            Relation::Direct(direct) => okta_fga::Userset {
                this: Some(okta_fga::DirectUserset {}),
                computed_userset: None,
                tuple_to_userset: None,
                union: Some(okta_fga::Usersets {
                    child: direct.into_iter().map(|d| d.into()).collect(),
                }),
                intersection: None,
                difference: None,
            },
            Relation::Indirect(indirect) => okta_fga::Userset {
                this: None,
                computed_userset: Some(indirect.into()),
                tuple_to_userset: None,
                union: None,
                intersection: None,
                difference: None,
            },
        }
    }
}

impl From<DirectRelation> for okta_fga::Userset {
    fn from(direct: DirectRelation) -> Self {
        match direct {
            DirectRelation::TupleString(s) => okta_fga::Userset {
                this: Some(okta_fga::DirectUserset {}),
                computed_userset: None,
                tuple_to_userset: None,
                union: None,
                intersection: None,
                difference: None,
            },
            DirectRelation::TypeRelation { type_, relation } => okta_fga::Userset {
                this: None,
                computed_userset: Some(okta_fga::ObjectRelation {
                    object: Some(type_),
                    relation: Some(relation),
                }),
                tuple_to_userset: None,
                union: None,
                intersection: None,
                difference: None,
            },
        }
    }
}

impl From<IndirectRelation> for okta_fga::ObjectRelation {
    fn from(indirect: IndirectRelation) -> Self {
        match indirect {
            IndirectRelation::Internal(s) => okta_fga::ObjectRelation {
                object: Some(s),
                relation: None,
            },
            IndirectRelation::External { relation, from } => okta_fga::ObjectRelation {
                object: Some(from),
                relation: Some(relation),
            },
        }
    }
}

impl From<ConditionalRelation> for okta_fga::Userset {
    fn from(conditional: ConditionalRelation) -> Self {
        okta_fga::Userset {
            this: None,
            computed_userset: None,
            tuple_to_userset: None,
            union: conditional.union.map(|u| okta_fga::Usersets {
                child: u.into_iter().map(|r| r.into()).collect(),
            }),
            intersection: conditional.intersection.map(|i| okta_fga::Usersets {
                child: i.into_iter().map(|r| r.into()).collect(),
            }),
            difference: conditional.exclusion.map(|e| okta_fga::V1Difference {
                base: Box::new(okta_fga::Userset {
                    this: None,
                    computed_userset: None,
                    tuple_to_userset: None,
                    union: Some(okta_fga::Usersets {
                        child: e.into_iter().map(|r| r.into()).collect(),
                    }),
                    intersection: None,
                    difference: None,
                }),
                subtract: Box::new(okta_fga::Userset {
                    this: None,
                    computed_userset: None,
                    tuple_to_userset: None,
                    union: None,
                    intersection: None,
                    difference: None,
                }),
            }),
        }
    }
}

impl From<RelationVariant> for okta_fga::Userset {
    fn from(variant: RelationVariant) -> Self {
        match variant {
            RelationVariant::Relation(relation) => relation.into(),
            RelationVariant::Conditional(conditional) => (*conditional).into(),
        }
    }
}
