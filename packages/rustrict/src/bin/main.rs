use std::collections::{HashMap, HashSet};

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
pub struct RelationConfig {
    pub name: String,
    pub userset_rewrite: Option<UsersetRewrite>,
}

#[derive(Debug, Clone)]
pub struct NamespaceConfig {
    pub name: String,
    pub relations: Vec<RelationConfig>,
}

impl NamespaceConfig {
    // Function to expand the namespace configuration into ACL tuples
    pub fn expand_to_acls(&self, object_id: &str, existing_acls: &HashSet<AclTuple>) -> HashSet<AclTuple> {
        let mut acls = HashSet::new();

        for relation in &self.relations {
            match &relation.userset_rewrite {
                Some(userset_rewrite) => {
                    for child in &userset_rewrite.union.children {
                        match child {
                            Child::This => {
                                // Add all direct users for this object#relation
                                for acl in existing_acls {
                                    if acl.object == format!("{}:{}", self.name, object_id) && acl.relation == relation.name {
                                        acls.insert(acl.clone());
                                    }
                                }
                            }
                            Child::ComputedUserset(computed_userset) => {
                                // Add all users for the computed userset relation
                                for acl in existing_acls {
                                    if acl.object == format!("{}:{}", self.name, object_id) && acl.relation == computed_userset.relation {
                                        acls.insert(AclTuple::new(
                                            format!("{}:{}", self.name, object_id),
                                            relation.name.clone(),
                                            acl.user.clone(),
                                        ));
                                    }
                                }
                            }
                            Child::TupleToUserset(tuple_to_userset) => {
                                // Add users from the tuple to userset relation
                                for acl in existing_acls {
                                    if acl.relation == tuple_to_userset.tupleset_relation {
                                        let user_object = acl.user.split('#').next().unwrap();
                                        let computed_acl = AclTuple::new(
                                            format!("{}:{}", user_object, object_id),
                                            tuple_to_userset.computed_userset.relation.clone(),
                                            acl.user.clone(),
                                        );
                                        acls.insert(computed_acl);
                                    }
                                }
                            }
                        }
                    }
                }
                None => {
                    // Add all direct users for this object#relation
                    for acl in existing_acls {
                        if acl.object == format!("{}:{}", self.name, object_id) && acl.relation == relation.name {
                            acls.insert(acl.clone());
                        }
                    }
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
        let existing_acls: HashSet<AclTuple> = vec![
            AclTuple::new("doc:123".to_string(), "owner".to_string(), "user1".to_string()),
            AclTuple::new("doc:123".to_string(), "editor".to_string(), "user2".to_string()),
            AclTuple::new("doc:456".to_string(), "viewer".to_string(), "user3".to_string()),
            AclTuple::new("parent:123".to_string(), "viewer".to_string(), "user4".to_string()),
        ]
            .into_iter()
            .collect();

        let namespace_config = NamespaceConfig {
            name: "doc".to_string(),
            relations: vec![
                RelationConfig {
                    name: "owner".to_string(),
                    userset_rewrite: None,
                },
                RelationConfig {
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
                RelationConfig {
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

        let acls = namespace_config.expand_to_acls("123", &existing_acls);
        let expected_acls: HashSet<AclTuple> = vec![
            AclTuple::new("doc:123".to_string(), "owner".to_string(), "user1".to_string()),
            AclTuple::new("doc:123".to_string(), "editor".to_string(), "user2".to_string()),
            AclTuple::new("doc:123".to_string(), "editor".to_string(), "user1".to_string()),
            AclTuple::new("doc:123".to_string(), "viewer".to_string(), "user2".to_string()),
            AclTuple::new("doc:123".to_string(), "viewer".to_string(), "user4".to_string()),
        ]
            .into_iter()
            .collect();

        assert_eq!(acls, expected_acls);
    }
}

fn main() {
    let existing_acls: HashSet<AclTuple> = vec![
        AclTuple::new("doc:123".to_string(), "owner".to_string(), "user1".to_string()),
        AclTuple::new("doc:123".to_string(), "editor".to_string(), "user2".to_string()),
        AclTuple::new("doc:456".to_string(), "viewer".to_string(), "user3".to_string()),
        AclTuple::new("parent:123".to_string(), "viewer".to_string(), "user4".to_string()),
    ]
        .into_iter()
        .collect();

    let namespace_config = NamespaceConfig {
        name: "doc".to_string(),
        relations: vec![
            RelationConfig {
                name: "owner".to_string(),
                userset_rewrite: None,
            },
            RelationConfig {
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
            RelationConfig {
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

    let acls = namespace_config.expand_to_acls("123", &existing_acls);
    for acl in acls {
        println!("{:?}", acl);
    }
}
