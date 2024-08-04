use rustrict::encoding::{from_protobuf, from_zanzibar_notation, to_protobuf, to_zanzibar_notation};
use rustrict::graph::{Entity, RelationGraph};
use rustrict::parsing::parse_relation_tuple;

use super::*;

fn equality_predicate(entity: &Entity, relation: &String) -> bool {
    true
}

fn url_path_predicate(entity: &Entity, relation: &String) -> bool {
    if relation == "url_paths" {
        entity.namespace == "users" && relation.starts_with("/resource")
    } else {
        false
    }
}

#[test]
fn test_relation_graph_basic() {
    let mut graph = RelationGraph::new();

    let tuples = vec![
        "file:foo.pdf#owner@user:alice",
        "file:foo.pdf#read@group:admin_group",
        "file:foo.pdf#write@group:admin_group",
        "file:bar.pdf#read@group:admin_group",
        "file:bar.pdf#delete@user:bob",
        "group:admin_group#member@user:alice",
        "group:admin_group#member@user:bob",
    ];

    for tuple in tuples {
        let relation_tuple = parse_relation_tuple(tuple);
        graph.add_relationship(relation_tuple);
    }

    let user = Entity::new("user", "alice");
    let relation = "read".to_string();

    assert!(graph.check_permission(&user, &relation, equality_predicate));
    assert!(!graph.check_permission(&user, &"delete".to_string(), equality_predicate));

    println!("Dot graph:\n{}", graph.generate_dot());
}

#[test]
fn test_relation_graph_complex() {
    let mut graph = RelationGraph::new();

    let tuples = vec![
        "file:foo.pdf#owner@user:alice",
        "file:foo.pdf#read@group:admin_group",
        "file:foo.pdf#write@group:admin_group",
        "file:bar.pdf#read@group:admin_group",
        "file:bar.pdf#delete@user:bob",
        "group:admin_group#member@user:alice",
        "group:admin_group#member@user:bob",
    ];

    for tuple in tuples {
        let relation_tuple = parse_relation_tuple(tuple);
        graph.add_relationship(relation_tuple);
    }

    let user1 = Entity::new("user", "alice");
    let user2 = Entity::new("user", "bob");

    assert!(graph.check_permission(&user1, &"read".to_string(), equality_predicate));
    assert!(graph.check_permission(&user1, &"write".to_string(), equality_predicate));
    assert!(!graph.check_permission(&user1, &"delete".to_string(), equality_predicate));
    assert!(graph.check_permission(&user2, &"delete".to_string(), equality_predicate));
    assert!(!graph.check_permission(&user2, &"write".to_string(), equality_predicate));

    println!("Dot graph:\n{}", graph.generate_dot());
}

#[test]
fn test_string_encoding() {
    let mut graph = RelationGraph::new();

    let tuples = vec![
        "file:foo.pdf#owner@user:alice",
        "file:foo.pdf#read@group:admin_group",
        "file:foo.pdf#write@group:admin_group",
        "file:bar.pdf#read@group:admin_group",
        "file:bar.pdf#delete@user:bob",
        "group:admin_group#member@user:alice",
        "group:admin_group#member@user:bob",
    ];

    for tuple in tuples {
        let relation_tuple = parse_relation_tuple(tuple);
        graph.add_relationship(relation_tuple);
    }

    let encoded = to_string_encoding(&graph);
    println!("String Encoding:\n{}", encoded);

    let decoded_graph = from_string_encoding(&encoded);

    let user1 = Entity::new("user", "alice");
    let user2 = Entity::new("user", "bob");

    assert!(decoded_graph.check_permission(&user1, &"read".to_string(), equality_predicate));
    assert!(decoded_graph.check_permission(&user1, &"write".to_string(), equality_predicate));
    assert!(!decoded_graph.check_permission(&user1, &"delete".to_string(), equality_predicate));
    assert!(decoded_graph.check_permission(&user2, &"delete".to_string(), equality_predicate));
    assert!(!decoded_graph.check_permission(&user2, &"write".to_string(), equality_predicate));
}

#[test]
fn test_protobuf_encoding() {
    let mut graph = RelationGraph::new();

    let tuples = vec![
        "file:foo.pdf#owner@user:alice",
        "file:foo.pdf#read@group:admin_group",
        "file:foo.pdf#write@group:admin_group",
        "file:bar.pdf#read@group:admin_group",
        "file:bar.pdf#delete@user:bob",
        "group:admin_group#member@user:alice",
        "group:admin_group#member@user:bob",
    ];

    for tuple in tuples {
        let relation_tuple = parse_relation_tuple(tuple);
        graph.add_relationship(relation_tuple);
    }

    let encoded = to_protobuf(&graph);
    println!("Protobuf Encoding:\n{:?}", encoded);

    let decoded_graph = from_protobuf(&encoded);

    let user1 = Entity::new("user", "alice");
    let user2 = Entity::new("user", "bob");

    assert!(decoded_graph.check_permission(&user1, &"read".to_string(), equality_predicate));
    assert!(decoded_graph.check_permission(&user1, &"write".to_string(), equality_predicate));
    assert!(!decoded_graph.check_permission(&user1, &"delete".to_string(), equality_predicate));
    assert!(decoded_graph.check_permission(&user2, &"delete".to_string(), equality_predicate));
    assert!(!decoded_graph.check_permission(&user2, &"write".to_string(), equality_predicate));
}
