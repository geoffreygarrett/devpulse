// // src/serialization.rs
//
// use petgraph::Direction;
// use prost::Message;
// use std::convert::TryFrom;
//
// use crate::graph::{NodeType, RelationGraph, RelationTuple};
// use crate::proto;
// use crate::errors::ProtobufError;
//
// pub fn to_protobuf(graph: &RelationGraph) -> Vec<u8> {
//     let entities: Vec<proto::Entity> = graph
//         .node_indices
//         .keys()
//         .filter_map(|node| {
//             if let NodeType::Entity(entity) = node {
//                 Some(entity.clone().into())
//             } else {
//                 None
//             }
//         })
//         .collect();
//
//     let relationships: Vec<proto::RelationTuple> = graph
//         .graph
//         .edge_indices()
//         .filter_map(|edge| {
//             let (source, target) = graph.graph.edge_endpoints(edge).unwrap();
//             if let (NodeType::Entity(object), NodeType::Relation(relation)) =
//                 (&graph.graph[source], &graph.graph[target])
//             {
//                 if let Some(user_index) = graph
//                     .graph
//                     .neighbors_directed(target, Direction::Outgoing)
//                     .next()
//                 {
//                     if let NodeType::Entity(user) = &graph.graph[user_index] {
//                         return Some(proto::RelationTuple {
//                             entity: Some(object.clone().into()),
//                             relation: relation.clone(),
//                             user: Some(proto::User {
//                                 user: match user {
//                                     NodeType::Entity(entity) => Some(proto::user::User::Id(
//                                         entity.id.clone(),
//                                     )),
//                                     _ => None,
//                                 },
//                             }),
//                         });
//                     }
//                 }
//             }
//             None
//         })
//         .collect();
//
//     let cache: Vec<proto::CacheEntry> = graph
//         .get_cache_entries()
//         .iter()
//         .map(|entry| entry.clone().into())
//         .collect();
//
//     let data = proto::GraphData {
//         entities,
//         relationships,
//         cache,
//     };
//
//     let mut buf = Vec::new();
//     data.encode(&mut buf).unwrap();
//     buf
// }
//
// pub fn from_protobuf(buf: &[u8]) -> Result<RelationGraph, ProtobufError> {
//     let decoded = proto::GraphData::decode(buf)?;
//     let mut graph = RelationGraph::new();
//
//     for entity in decoded.entities {
//         graph.add_entity(Entity::try_from(entity)?);
//     }
//
//     for relationship in decoded.relationships {
//         graph.add_relationship(RelationTuple::try_from(relationship)?);
//     }
//
//     for entry in decoded.cache {
//         graph.add_cache_entry(proto::CacheEntry::try_from(entry)?);
//     }
//
//     Ok(graph)
// }
//
// pub fn to_zanzibar_notation(graph: &RelationGraph) -> String {
//     let mut output = String::new();
//
//     for (node, _) in &graph.node_indices {
//         if let NodeType::Entity(entity) = node {
//             output.push_str(&format!("{}:{}\n", entity.namespace, entity.id));
//         }
//     }
//
//     for edge in graph.graph.edge_indices() {
//         let (source, target) = graph.graph.edge_endpoints(edge).unwrap();
//         if let (NodeType::Entity(object), NodeType::Relation(relation)) =
//             (&graph.graph[source], &graph.graph[target])
//         {
//             if let Some(user_index) = graph
//                 .graph
//                 .neighbors_directed(target, Direction::Outgoing)
//                 .next()
//             {
//                 if let NodeType::Entity(user) = &graph.graph[user_index] {
//                     output.push_str(&format!(
//                         "{}:{}#{}@{}:{}\n",
//                         object.namespace, object.id, relation, user.namespace, user.id
//                     ));
//                 }
//             }
//         }
//     }
//
//     for entry in graph.get_cache_entries() {
//         output.push_str(&format!(
//             "CACHE: {}:{}#{} -> {}\n",
//             entry.entity.namespace, entry.entity.id, entry.relation, entry.result
//         ));
//     }
//
//     output
// }
//
// pub fn from_zanzibar_notation(input: &str) -> RelationGraph {
//     let mut graph = RelationGraph::new();
//     let lines: Vec<&str> = input.lines().collect();
//
//     for line in lines {
//         if line.starts_with("CACHE:") {
//             let parts: Vec<&str> = line[6..].split(" -> ").collect();
//             let cache_key: Vec<&str> = parts[0].split('#').collect();
//             let entity_parts: Vec<&str> = cache_key[0].split(':').collect();
//             let entity = Entity::new(entity_parts[0], entity_parts[1]);
//             let relation = cache_key[1].to_string();
//             let result = parts[1] == "true";
//             graph.add_cache_entry(proto::CacheEntry {
//                 entity,
//                 relation,
//                 result,
//             });
//         } else if line.contains('#') && line.contains('@') {
//             let parts: Vec<&str> = line.split('#').collect();
//             let object_parts: Vec<&str> = parts[0].split(':').collect();
//             let user_parts: Vec<&str> = parts[1].split('@').collect();
//             let relation = user_parts[0].to_string();
//             let user_entity_parts: Vec<&str> = user_parts[1].split(':').collect();
//             let object = Entity::new(object_parts[0], object_parts[1]);
//             let user = Entity::new(user_entity_parts[0], user_entity_parts[1]);
//             graph.add_relationship(RelationTuple {
//                 entity: object,
//                 relation,
//                 user,
//             });
//         } else {
//             let parts: Vec<&str> = line.split(':').collect();
//             graph.add_entity(Entity::new(parts[0], parts[1]));
//         }
//     }
//
//     graph
// }
