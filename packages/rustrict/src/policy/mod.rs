use std::collections::HashSet;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AclTuple {
    pub object: String,
    pub relation: String,
    pub user: String,
}

impl AclTuple {
    pub fn new(object: String, relation: String, user: String) -> Self {
        Self {
            object,
            relation,
            user,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ComputedUserset {
    pub relation: String,
}

#[derive(Debug, Clone)]
pub struct TupleToUserset {
    pub tupleset_relation: String,
    pub computed_userset: ComputedUserset,
}

#[derive(Debug, Clone)]
pub enum Child {
    This,
    ComputedUserset(ComputedUserset),
    TupleToUserset(TupleToUserset),
}

#[derive(Debug, Clone)]
pub struct Union {
    pub children: Vec<Child>,
}

#[derive(Debug, Clone)]
pub struct UsersetRewrite {
    pub union: Union,
}

#[derive(Debug, Clone)]
pub struct Relation {
    pub name: String,
    pub userset_rewrite: Option<UsersetRewrite>,
}

#[derive(Debug, Clone)]
pub struct Policy {
    pub name: String,
    pub relations: Vec<Relation>,
}

impl Policy {
    // Function to expand the policy structure into ACL tuples
    pub fn expand_to_acls(&self, object_id: &str) -> HashSet<AclTuple> {
        let mut acls = HashSet::new();

        for relation in &self.relations {
            match &relation.userset_rewrite {
                Some(userset_rewrite) => {
                    for child in &userset_rewrite.union.children {
                        match child {
                            Child::This => {
                                acls.insert(AclTuple::new(
                                    format!("{}:{}", self.name, object_id),
                                    relation.name.clone(),
                                    "direct_user".to_string(), // Placeholder for direct users
                                ));
                            }
                            Child::ComputedUserset(computed_userset) => {
                                acls.insert(AclTuple::new(
                                    format!("{}:{}", self.name, object_id),
                                    relation.name.clone(),
                                    format!("{}#{}", self.name, computed_userset.relation),
                                ));
                            }
                            Child::TupleToUserset(tuple_to_userset) => {
                                acls.insert(AclTuple::new(
                                    format!("{}:{}", self.name, object_id),
                                    relation.name.clone(),
                                    format!(
                                        "{}#{}",
                                        tuple_to_userset.tupleset_relation,
                                        tuple_to_userset.computed_userset.relation
                                    ),
                                ));
                            }
                        }
                    }
                }
                None => {
                    acls.insert(AclTuple::new(
                        format!("{}:{}", self.name, object_id),
                        relation.name.clone(),
                        "direct_user".to_string(), // Placeholder for direct users
                    ));
                }
            }
        }

        acls
    }
}

// Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_expand_to_acls() {
        let policy = Policy {
            name: "doc".to_string(),
            relations: vec![
                Relation {
                    name: "owner".to_string(),
                    userset_rewrite: None,
                },
                Relation {
                    name: "editor".to_string(),
                    userset_rewrite: Some(UsersetRewrite {
                        union: Union {
                            children: vec![
                                Child::This,
                                Child::ComputedUserset(ComputedUserset {
                                    relation: "owner".to_string(),
                                }),
                            ],
                        },
                    }),
                },
                Relation {
                    name: "viewer".to_string(),
                    userset_rewrite: Some(UsersetRewrite {
                        union: Union {
                            children: vec![
                                Child::This,
                                Child::ComputedUserset(ComputedUserset {
                                    relation: "editor".to_string(),
                                }),
                                Child::TupleToUserset(TupleToUserset {
                                    tupleset_relation: "parent".to_string(),
                                    computed_userset: ComputedUserset {
                                        relation: "viewer".to_string(),
                                    },
                                }),
                            ],
                        },
                    }),
                },
            ],
        };

        let acls = policy.expand_to_acls("123");
        let expected_acls: HashSet<AclTuple> = vec![
            AclTuple::new("doc:123".to_string(), "owner".to_string(), "direct_user".to_string()),
            AclTuple::new("doc:123".to_string(), "editor".to_string(), "direct_user".to_string()),
            AclTuple::new("doc:123".to_string(), "editor".to_string(), "doc#owner".to_string()),
            AclTuple::new("doc:123".to_string(), "viewer".to_string(), "direct_user".to_string()),
            AclTuple::new("doc:123".to_string(), "viewer".to_string(), "doc#editor".to_string()),
            AclTuple::new("doc:123".to_string(), "viewer".to_string(), "parent#viewer".to_string()),
        ]
        .into_iter()
        .collect();

        assert_eq!(acls, expected_acls);
    }
}

fn main() {
    let policy = Policy {
        name: "doc".to_string(),
        relations: vec![
            Relation {
                name: "owner".to_string(),
                userset_rewrite: None,
            },
            Relation {
                name: "editor".to_string(),
                userset_rewrite: Some(UsersetRewrite {
                    union: Union {
                        children: vec![
                            Child::This,
                            Child::ComputedUserset(ComputedUserset {
                                relation: "owner".to_string(),
                            }),
                        ],
                    },
                }),
            },
            Relation {
                name: "viewer".to_string(),
                userset_rewrite: Some(UsersetRewrite {
                    union: Union {
                        children: vec![
                            Child::This,
                            Child::ComputedUserset(ComputedUserset {
                                relation: "editor".to_string(),
                            }),
                            Child::TupleToUserset(TupleToUserset {
                                tupleset_relation: "parent".to_string(),
                                computed_userset: ComputedUserset {
                                    relation: "viewer".to_string(),
                                },
                            }),
                        ],
                    },
                }),
            },
        ],
    };

    let acls = policy.expand_to_acls("123");
    for acl in acls {
        println!("{:?}", acl);
    }
}
