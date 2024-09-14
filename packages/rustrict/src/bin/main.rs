use std::collections::{HashMap, HashSet};

use serde::{Deserialize, Serialize};

use rustrict::models::config::{ModelConfig, UsersetRewrite};

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Hash, Clone)]
pub struct RelationshipTuple {
    pub user: String,
    pub relation: String,
    pub object: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct RelationshipTuples {
    pub tuples: Vec<RelationshipTuple>,
}

pub enum TupleOperation {
    #[serde(rename = "TUPLE_OPERATION_WRITE")]
    Write,
    #[serde(rename = "TUPLE_OPERATION_DELETE")]
    Delete,
}

pub struct TupleKey {
    pub user: String,
    pub relation: String,
    pub object: String,
    pub condition: Option<String>,
}
struct TupleKeyChanges {
    tuple_key: TupleKey,
    operation: TupleOperation,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
enum TypeNames {
    #[serde(rename = "TYPE_NAME_UNSPECIFIED")]
    Unspecified,
    #[serde(rename = "TYPE_NAME_ANY")]
    Any,
    #[serde(rename = "TYPE_NAME_BOOL")]
    Boolean,
    #[serde(rename = "TYPE_NAME_STRING")]
    String,
    #[serde(rename = "TYPE_NAME_INT")]
    Integer,
    #[serde(rename = "TYPE_NAME_UINT")]
    UInteger,
    #[serde(rename = "TYPE_NAME_DOUBLE")]
    Double,
    #[serde(rename = "TYPE_NAME_DURATION")]
    Duration,
    #[serde(rename = "TYPE_NAME_TIMESTAMP")]
    Timestamp,
    #[serde(rename = "TYPE_NAME_MAP")]
    Map,
    #[serde(rename = "TYPE_NAME_LIST")]
    List,
    #[serde(rename = "TYPE_NAME_IPADDRESS")]
    IpAddress,
}

impl RelationshipTuples {
    pub fn new() -> Self {
        RelationshipTuples { tuples: Vec::new() }
    }

    pub fn add_relationship_tuple(&mut self, tuple: RelationshipTuple) -> Result<(), String> {
        self.validate_relationship_tuple(&tuple)?;
        self.tuples.push(tuple);
        Ok(())
    }

    fn validate_relationship_tuple(&self, tuple: &RelationshipTuple) -> Result<(), String> {
        // Implement validation logic (e.g., check if the relation is valid for the object type)
        Ok(())
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Hash, Clone)]
pub struct ACL {
    pub user: String,
    pub object: String,
    pub relation: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct ACLs {
    pub acls: HashSet<ACL>,
    pub membership_cache: HashMap<String, HashSet<String>>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct UsersetTree {
    pub root: Node,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Node {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub union: Option<Nodes>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub intersection: Option<Nodes>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclusion: Option<Nodes>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub leaf: Option<Leaf>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Nodes {
    pub nodes: Vec<Node>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Computed {
    pub userset: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Leaf {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub users: Option<HashSet<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub computed: Option<Computed>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tuple_to_userset: Option<String>,
}

impl ACLs {
    pub fn new() -> Self {
        ACLs {
            acls: HashSet::new(),
            membership_cache: HashMap::new(),
        }
    }

    pub fn expand_userset_expression_tree(
        &self,
        object: &str,
        relation: &str,
        model_config: &ModelConfig,
        relationship_tuples: &RelationshipTuples,
    ) -> Result<UsersetTree, String> {
        for type_def in &model_config.types {
            if let Some(relations) = &type_def.relations {
                if let Some(userset_rewrite) = relations.get(relation) {
                    let root_node = self.build_userset_node(
                        userset_rewrite,
                        &format!("{}#{}", object, relation),
                        object,
                        relation,
                        relationship_tuples,
                    )?;
                    return Ok(UsersetTree { root: root_node });
                }
            }
        }
        Err("Relation not found in model config".to_string())
    }

    fn build_userset_node(
        &self,
        userset_rewrite: &UsersetRewrite,
        node_type: &str,
        object: &str,
        relation: &str,
        relationship_tuples: &RelationshipTuples,
    ) -> Result<Node, String> {
        let mut node = Node {
            name: node_type.to_string(),
            union: None,
            intersection: None,
            exclusion: None,
            leaf: None,
        };

        if let Some(_) = &userset_rewrite.this {
            let mut users = HashSet::new();
            for tuple in &relationship_tuples.tuples {
                if tuple.object == object && tuple.relation == relation {
                    users.insert(tuple.user.clone());
                }
            }
            node.leaf = Some(Leaf {
                users: Some(users),
                computed: None,
                tuple_to_userset: None,
            });
        }

        if let Some(object_relation) = &userset_rewrite.computed_userset {
            node.leaf = Some(Leaf {
                users: None,
                computed: Some(Computed {
                    userset: format!(
                        "{}#{}",
                        object,
                        object_relation.relation.clone().unwrap_or_default()
                    ),
                }),
                tuple_to_userset: None,
            });
        }

        if let Some(tuple_to_userset) = &userset_rewrite.tuple_to_userset {
            node.leaf = Some(Leaf {
                users: None,
                computed: None,
                tuple_to_userset: Some(format!(
                    "{}#{}",
                    object,
                    tuple_to_userset.computed_userset.relation.clone().unwrap_or_default()
                )),
            });
        }

        if let Some(union) = &userset_rewrite.union {
            let mut nodes = Vec::new();
            for child in union {
                let child_node = self.build_userset_node(
                    child,
                    node_type,
                    object,
                    relation,
                    relationship_tuples,
                )?;
                nodes.push(child_node);
            }
            node.union = Some(Nodes { nodes });
        }

        if let Some(intersection) = &userset_rewrite.intersection {
            let mut nodes = Vec::new();
            for child in intersection {
                let child_node = self.build_userset_node(
                    child,
                    node_type,
                    object,
                    relation,
                    relationship_tuples,
                )?;
                nodes.push(child_node);
            }
            node.intersection = Some(Nodes { nodes });
        }

        if let Some(exclusion) = &userset_rewrite.exclusion {
            let mut nodes = Vec::new();
            for child in exclusion {
                let child_node = self.build_userset_node(
                    child,
                    node_type,
                    object,
                    relation,
                    relationship_tuples,
                )?;
                nodes.push(child_node);
            }
            node.exclusion = Some(Nodes { nodes });
        }

        Ok(node)
    }

    pub fn check(
        &self,
        user: &str,
        object: &str,
        relation: &str,
        model_config: &ModelConfig,
        relationship_tuples: &RelationshipTuples,
    ) -> Result<bool, String> {
        let tree = self.expand_userset_expression_tree(object, relation, model_config, relationship_tuples)?;
        Ok(self.evaluate_node(&tree.root, user, model_config, relationship_tuples))
    }

    fn evaluate_node(
        &self,
        node: &Node,
        user: &str,
        model_config: &ModelConfig,
        relationship_tuples: &RelationshipTuples,
    ) -> bool {
        if let Some(leaf) = &node.leaf {
            if let Some(users) = &leaf.users {
                if users.contains(user) {
                    return true;
                }
            }

            if let Some(computed) = &leaf.computed {
                let parts: Vec<&str> = computed.userset.split('#').collect();
                if parts.len() == 2 {
                    let object = parts[0];
                    let relation = parts[1];
                    return self.check(user, object, relation, model_config, relationship_tuples).unwrap_or(false);
                }
            }
        }

        if let Some(union) = &node.union {
            for child_node in &union.nodes {
                if self.evaluate_node(child_node, user, model_config, relationship_tuples) {
                    return true;
                }
            }
        }

        if let Some(intersection) = &node.intersection {
            for child_node in &intersection.nodes {
                if !self.evaluate_node(child_node, user, model_config, relationship_tuples) {
                    return false;
                }
            }
            return true;
        }

        if let Some(exclusion) = &node.exclusion {
            if self.evaluate_node(&exclusion.nodes[0], user, model_config, relationship_tuples) {
                for i in 1..exclusion.nodes.len() {
                    if self.evaluate_node(&exclusion.nodes[i], user, model_config, relationship_tuples) {
                        return false;
                    }
                }
                return true;
            }
        }

        false
    }
}

// tuple_key:
// $ref: '#/definitions/CheckRequestTupleKey'
// contextual_tuples:
// $ref: '#/definitions/ContextualTupleKeys'


#[cfg(test)]
mod tests {
    use super::*;

    fn setup_model_config_roles_and_permissions() -> ModelConfig {
        let model_config_str = r#"
        model
          schema 1.1

        type user

        type trip
          relations
            define owner: [user]
            define viewer: [user]
            define booking_adder: owner
            define booking_viewer: viewer or owner
        "#;

        // 03. Checking User Roles And Their Permissions
        // Now that your type definitions reflect the roles and permissions governing how bookings can be viewed and added, create relationship tuples to assign roles to users, then check if users have the proper permissions.
        //
        //     Create two relationship tuples:
        //
        //     gives bob the role of viewer on trip:Europe.
        //     gives alice the role of owner on trip:Europe.


        ModelConfig::from_fga_file_contents(model_config_str).unwrap()
    }

    fn setup_relationship_tuples_roles_and_permissions() -> RelationshipTuples {
        let mut relationship_tuples = RelationshipTuples::new();

        let tuples = vec![
            RelationshipTuple {
                user: "user:bob".to_string(),
                relation: "viewer".to_string(),
                object: "trip:Europe".to_string(),
                _description: Some("Add bob as viewer on trip:Europe".to_string()),
            },
            RelationshipTuple {
                user: "user:alice".to_string(),
                relation: "owner".to_string(),
                object: "trip:Europe".to_string(),
                _description: Some("Add alice as owner on trip:Europe".to_string()),
            },
        ];

        for tuple in tuples {
            relationship_tuples.add_relationship_tuple(tuple).unwrap();
        }

        relationship_tuples
    }

    fn setup_model_config() -> ModelConfig {
        let model_config_str = r#"
        model
          schema 1.1

        type user

        type org
          relations
            define member: [user]

        type document
          relations
            define writer: [user, org#member]
            define reader: [user, org#member] or writer
        "#;

        ModelConfig::from_fga_file_contents(model_config_str).unwrap()
    }

    #[test]
    fn test_is_bob_allowed_to_view_bookies_on_trip_europe() {
        let model_config = setup_model_config_roles_and_permissions();
        let relationship_tuples = setup_relationship_tuples_roles_and_permissions();
        let acls = ACLs::new();

        let result = acls.check("user:bob", "trip:Europe", "booking_viewer", &model_config, &relationship_tuples);
        assert_eq!(result.unwrap(), true);
    }
    #[test]
    fn test_is_bob_allowed_to_add_bookies_on_trip_europe() {
        let model_config = setup_model_config_roles_and_permissions();
        let relationship_tuples = setup_relationship_tuples_roles_and_permissions();
        let acls = ACLs::new();

        let result = acls.check("user:bob", "trip:Europe", "booking_adder", &model_config, &relationship_tuples);
        assert_eq!(result.unwrap(), false);
    }
    #[test]
    fn test_is_alice_allowed_to_view_and_add_bookies_on_trip_europe() {
        let model_config = setup_model_config_roles_and_permissions();
        let relationship_tuples = setup_relationship_tuples_roles_and_permissions();
        let acls = ACLs::new();

        let result = acls.check("user:alice", "trip:Europe", "booking_viewer", &model_config, &relationship_tuples);
        assert_eq!(result.unwrap(), true);

        let result = acls.check("user:alice", "trip:Europe", "booking_adder", &model_config, &relationship_tuples);
        assert_eq!(result.unwrap(), true);
    }
    fn setup_relationship_tuples() -> RelationshipTuples {
        let mut relationship_tuples = RelationshipTuples::new();

        let tuples = vec![
            RelationshipTuple {
                user: "user:bob".to_string(),
                relation: "member".to_string(),
                object: "org:1".to_string(),
                _description: Some("Bob is a member of organization 1".to_string()),
            },
            RelationshipTuple {
                user: "org:1".to_string(),
                relation: "reader".to_string(),
                object: "document:budget".to_string(),
                _description: Some("Organization 1 is a reader of the budget document".to_string()),
            },
            RelationshipTuple {
                user: "user:alice".to_string(),
                relation: "writer".to_string(),
                object: "document:budget".to_string(),
                _description: Some("Alice is a writer of the budget document".to_string()),
            },
            RelationshipTuple {
                user: "user:charlie".to_string(),
                relation: "reader".to_string(),
                object: "document:budget".to_string(),
                _description: Some("Charlie is a reader of the budget document".to_string()),
            },
            RelationshipTuple {
                user: "user:david".to_string(),
                relation: "member".to_string(),
                object: "org:2".to_string(),
                _description: Some("David is a member of organization 2".to_string()),
            },
            RelationshipTuple {
                user: "user:eve".to_string(),
                relation: "writer".to_string(),
                object: "document:report".to_string(),
                _description: Some("Eve is a writer of the report document".to_string()),
            },
        ];

        for tuple in tuples {
            relationship_tuples.add_relationship_tuple(tuple).unwrap();
        }

        relationship_tuples
    }

    #[test]
    fn test_reader_access() {
        let model_config = setup_model_config();
        let relationship_tuples = setup_relationship_tuples();
        let acls = ACLs::new();

        let result = acls.check("user:charlie", "document:budget", "reader", &model_config, &relationship_tuples);
        assert_eq!(result.unwrap(), true);
    }

    #[test]
    fn test_writer_inherits_reader_access() {
        let model_config = setup_model_config();
        let relationship_tuples = setup_relationship_tuples();
        let acls = ACLs::new();

        let result = acls.check("user:alice", "document:budget", "reader", &model_config, &relationship_tuples);
        assert_eq!(result.unwrap(), true);
    }

    #[test]
    fn test_non_member_access() {
        let model_config = setup_model_config();
        let relationship_tuples = setup_relationship_tuples();
        let acls = ACLs::new();

        let result = acls.check("user:david", "document:budget", "reader", &model_config, &relationship_tuples);
        assert_eq!(result.unwrap(), false);
    }

    #[test]
    fn test_writer_access() {
        let model_config = setup_model_config();
        let relationship_tuples = setup_relationship_tuples();
        let acls = ACLs::new();

        let result = acls.check("user:eve", "document:report", "writer", &model_config, &relationship_tuples);
        assert_eq!(result.unwrap(), true);
    }

    #[test]
    fn test_writer_does_not_inherit_access() {
        let model_config = setup_model_config();
        let relationship_tuples = setup_relationship_tuples();
        let acls = ACLs::new();

        let result = acls.check("user:eve", "document:report", "reader", &model_config, &relationship_tuples);
        assert_eq!(result.unwrap(), false);
    }

    #[test]
    fn test_member_inherits_writer_access() {
        let model_config = setup_model_config();
        let mut relationship_tuples = setup_relationship_tuples();

        let additional_tuple = RelationshipTuple {
            user: "user:frank".to_string(),
            relation: "member".to_string(),
            object: "org:1".to_string(),
            _description: Some("Frank is a member of organization 1".to_string()),
        };
        relationship_tuples.add_relationship_tuple(additional_tuple).unwrap();
        let acls = ACLs::new();
        let result = acls.check("user:frank", "document:budget", "writer", &model_config, &relationship_tuples);
        assert_eq!(result.unwrap(), true);
    }

    #[test]
    fn test_member_inherits_reader_access() {
        let model_config = setup_model_config();
        let mut relationship_tuples = setup_relationship_tuples();

        let additional_tuple = RelationshipTuple {
            user: "user:frank".to_string(),
            relation: "member".to_string(),
            object: "org:1".to_string(),
            _description: Some("Frank is a member of organization 1".to_string()),
        };
        relationship_tuples.add_relationship_tuple(additional_tuple).unwrap();
        let acls = ACLs::new();
        let result = acls.check("user:frank", "document:budget", "reader", &model_config, &relationship_tuples);
        assert_eq!(result.unwrap(), true);
    }

    #[test]
    fn test_member_inherits_reader_access_across_docs() {
        let model_config = setup_model_config();
        let mut relationship_tuples = setup_relationship_tuples();

        let additional_tuple = RelationshipTuple {
            user: "user:grace".to_string(),
            relation: "member".to_string(),
            object: "org:1".to_string(),
            _description: Some("Grace is a member of organization 1".to_string()),
        };
        relationship_tuples.add_relationship_tuple(additional_tuple).unwrap();
        let acls = ACLs::new();

        let result = acls.check("user:grace", "document:budget", "reader", &model_config, &relationship_tuples);
        assert_eq!(result.unwrap(), true);
    }

    #[test]
    fn test_writer_inherits_reader_access_for_different_doc() {
        let model_config = setup_model_config();
        let relationship_tuples = setup_relationship_tuples();
        let acls = ACLs::new();

        let result = acls.check("user:eve", "document:budget", "reader", &model_config, &relationship_tuples);
        assert_eq!(result.unwrap(), false);
    }

    #[test]
    fn test_member_of_different_org_access() {
        let model_config = setup_model_config();
        let relationship_tuples = setup_relationship_tuples();
        let acls = ACLs::new();

        let result = acls.check("user:david", "document:budget", "writer", &model_config, &relationship_tuples);
        assert_eq!(result.unwrap(), false);
    }

    #[test]
    fn test_direct_writer_access() {
        let model_config = setup_model_config();
        let relationship_tuples = setup_relationship_tuples();
        let acls = ACLs::new();

        let result = acls.check("user:alice", "document:budget", "writer", &model_config, &relationship_tuples);
        assert_eq!(result.unwrap(), true);
    }

    #[test]
    fn test_direct_reader_access() {
        let model_config = setup_model_config();
        let relationship_tuples = setup_relationship_tuples();
        let acls = ACLs::new();

        let result = acls.check("user:charlie", "document:budget", "reader", &model_config, &relationship_tuples);
        assert_eq!(result.unwrap(), true);
    }

    #[test]
    fn test_invalid_user_access() {
        let model_config = setup_model_config();
        let relationship_tuples = setup_relationship_tuples();
        let acls = ACLs::new();

        let result = acls.check("user:unknown", "document:budget", "reader", &model_config, &relationship_tuples);
        assert_eq!(result.unwrap(), false);
    }
}
