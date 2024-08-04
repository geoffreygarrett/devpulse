// use std::collections::{HashMap, HashSet};
// use std::hash::Hash;
//
// use petgraph::dot::{Config, Dot};
// use petgraph::prelude::{DiGraph, EdgeRef, NodeIndex};
// use petgraph::visit::{Visitable, VisitMap};
//
// use crate::index::GroupIndex;
// use crate::models::zanzibar::{Object, Relationship, Subject};
// use crate::traits::{ToDot, ToMermaid};
//
// pub(crate) type Relation = String;
//
// #[derive(Debug, Clone, Hash, Eq, PartialEq)]
// pub(crate) enum Node<U, G, O>
// where
//     U: Eq + Hash,
//     G: Eq + Hash,
//     O: Eq + Hash,
// {
//     User(U),
//     Group(G),
//     Object(O),
// }
//
// impl<U, G, O> std::fmt::Display for Node<U, G, O>
// where
//     U: Eq + Hash + std::fmt::Display,
//     G: Eq + Hash + std::fmt::Display,
//     O: Eq + Hash + std::fmt::Display,
// {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         match self {
//             Node::User(user) => write!(f, "{}", user),
//             Node::Group(group) => write!(f, "{}", group),
//             Node::Object(object) => write!(f, "{}", object),
//         }
//     }
// }
//
// #[derive(Debug)]
// pub struct RelationGraph<'a, U, G, O>
// where
//     U: Eq + Hash + Clone + std::fmt::Display,
//     G: Eq + Hash + Clone + std::fmt::Display,
//     O: Eq + Hash + Clone + std::fmt::Display,
// {
//     graph: DiGraph<Node<U, G, O>, Relation>,
//     node_indices: HashMap<Node<U, G, O>, NodeIndex>,
//     group_index: GroupIndex<'a, U, G>,
// }
//
// impl<U, G, O> ToMermaid for RelationGraph<'_, U, G, O>
// where
//     U: std::fmt::Display + std::cmp::Eq + std::hash::Hash + Clone,
//     G: std::fmt::Display + std::cmp::Eq + std::hash::Hash + Clone,
//     O: std::fmt::Display + std::cmp::Eq + std::hash::Hash + Clone,
// {
//     fn to_mermaid(&self) -> String {
//         let mut mermaid = String::new();
//         mermaid.push_str("graph TD;\n");
//         mermaid.push_str("subgraph Objects\n");
//         for node in self.graph.node_indices() {
//             let object = &self.graph[node];
//             mermaid.push_str(&format!("    {}[{}]\n", node.index(), object));
//         }
//         mermaid.push_str("\n");
//         for edge in self.graph.edge_references() {
//             let source = edge.source().index();
//             let target = edge.target().index();
//             let relation = edge.weight();
//             mermaid.push_str(&format!("    {}--\"{}\"-->{}\n", source, relation, target));
//         }
//         mermaid.push_str("end\n");
//         mermaid
//     }
// }
//
// impl<U, G, O> ToDot for RelationGraph<'_, U, G, O>
// where
//     U: std::fmt::Display + std::cmp::Eq + std::hash::Hash + Clone,
//     G: std::fmt::Display + std::cmp::Eq + std::hash::Hash + Clone,
//     O: std::fmt::Display + std::cmp::Eq + std::hash::Hash + Clone,
// {
//     fn to_dot(&self) -> String {
//         Dot::with_attr_getters(
//             &self.graph,
//             &[Config::NodeNoLabel, Config::EdgeNoLabel],
//             &|_, edge| format!("label = {:?}", edge.weight()),
//             &|_, (node, object)| format!("label = {:?}", object.to_string()),
//         )
//         .to_string()
//     }
// }
//
// impl<'g, U, G, O> RelationGraph<'g, U, G, O>
// where
//     U: Eq + Hash + Clone + std::fmt::Display + std::fmt::Debug,
//     G: Eq + Hash + Clone + std::fmt::Display + std::fmt::Debug,
//     O: Eq + Hash + Clone + std::fmt::Display + std::fmt::Debug,
// {
//     pub fn new() -> Self {
//         RelationGraph {
//             graph: DiGraph::new(),
//             node_indices: HashMap::new(),
//             group_index: GroupIndex::new(),
//         }
//     }
//
//     pub fn add_node(&mut self, node: Node<U, G, O>) -> NodeIndex {
//         *self
//             .node_indices
//             .entry(node.clone())
//             .or_insert_with(|| self.graph.add_node(node))
//     }
//
//     pub fn add_relation(&mut self, subject: U, relation: Relation, object: O) {
//         let subject_node = Node::User(subject);
//         let object_node = Node::Object(object);
//         let subject_index = self.add_node(subject_node);
//         let object_index = self.add_node(object_node);
//         self.graph.add_edge(subject_index, object_index, relation);
//     }
//
//     // This function is now generic and makes use of the GroupIndex
//     pub async fn update_group_relations(&self, group: &'g G, subgroups: HashSet<&'g G>) {
//         self.group_index
//             .update_group_to_group(&group, subgroups)
//             .await;
//     }
//
//     pub fn collect_user_groups_from_graph(&self) -> HashMap<&'g U, HashSet<&'g G>> {
//         let mut user_groups = HashMap::new();
//         for (node, index) in &self.node_indices {
//             if let Node::User(user) = node {
//                 let mut groups = HashSet::new();
//                 for edge in self.graph.edges(*index) {
//                     if let Node::Group(group) = self.graph[&edge.target()] {
//                         groups.insert(group);
//                     }
//                 }
//                 user_groups.insert(user, groups);
//             }
//         }
//         user_groups
//     }
//
//     // MEMBER2GROUP(s) → {e}, where s represents an individual user and e represents a parent group
//     // in which the user is a direct member.
//     pub fn member_to_group(&self, user: &'g U) -> Option<Vec<G>> {
//         self.group_index.member_to_group(user)
//     }
//
//     // GROUP2GROUP(s) → {e}, where s represents an ancestor group and e represents a descendent
//     // group that is directly or indirectly a subgroup of the ancestor group.
//     pub fn group_to_group(&self, group: &'g G) -> Vec<G> {
//         self.group_index.group_to_group(group)
//     }
//
//     // (MEMBER2GROUP(U) ∩ GROUP2GROUP(G)) ̸= /0, Check if a user is a member of a group
//     pub fn is_member_of_group(&self, user: &U, group: &G) -> bool {
//         self.group_index.is_member_of_group(user, group)
//     }
//
//     /// Checks if a user U has a specific access level to an object based on Zanzibar's ACL model.
//     ///
//     /// The evaluation determines whether a direct relation tuple exists that grants the user `U`
//     /// the specified relation on the object, or if there exists a tuple that indirectly grants the
//     /// relation through another userset `U'`, for which the access must recursively be checked.
//     ///
//     /// This check can be expressed with the following formula:
//     ///
//     /// ```math
//     /// \text{CHECK}(U, \langle \text{object\#relation} \rangle) =
//     /// \exists \text{ tuple } \langle \text{object\#relation@U} \rangle \quad \lor \quad
//     /// \exists \text{ tuple } \langle \text{object\#relation@} U' \rangle
//     /// ```
//     ///
//     /// where `U'` is defined as:
//     ///
//     /// ```math
//     /// U' = \langle \text{object}'\#\text{relation}' \rangle
//     /// ```
//     ///
//     /// and it must satisfy:
//     ///
//     /// ```math
//     /// \text{CHECK}(U, U')
//     /// ```
//     ///
//     /// This method involves recursively checking access for `U` through any indirect ACLs or
//     /// group memberships.
//     ///
//     /// # References
//     /// - [Zanzibar: Google's Consistent, Global Authorization System](https://research.google/pubs/pub48190/), (3.2.3 Check Evaluation)
//     pub fn check(&self, user: &U, relation: &str, object: &O) -> bool {
//         let user_node = Node::User(user.clone());
//         let object_node = Node::Object(object.clone());
//
//         if let (Some(user_index), Some(object_index)) =
//             (self.node_indices.get(&user_node), self.node_indices.get(&object_node))
//         {
//             // Check direct access
//             if self.has_direct_access(*user_index, relation, *object_index) {
//                 return true;
//             }
//
//             // Check inherited access through groups
//             if let Some(groups) = self.member_to_group(user) {
//                 for group in groups {
//                     let group_node = Node::Group(group);
//                     if let Some(group_index) = self.node_indices.get(&group_node) {
//                         if self.has_direct_access(*group_index, relation, *object_index) {
//                             return true;
//                         }
//                     }
//                 }
//             }
//         }
//
//         false
//     }
//
//     /// Checks if a subject has a direct relation with an object.
//     fn has_direct_access(&self, subject: NodeIndex, relation: &str, object: NodeIndex) -> bool {
//         for edge in self.graph.edges(subject) {
//             if edge.weight() == relation && edge.target() == object {
//                 return true;
//             }
//         }
//         false
//     }
// }
//
// #[cfg(test)]
// mod tests {
//     use crate::models::zanzibar::{Object, Subject};
//
//     use super::*;
//
//     fn setup_document_graph() -> RelationGraph<'static, String, String, String> {
//         let mut graph = RelationGraph::new();
//         let relationships = [
//             "organization:contoso#member@anne",
//             "organization:fabrikam#member@beth",
//             "document:1#owner@anne",
//             "document:1#editor@organization:fabrikam#member",
//             "document:1#editor@document:1#owner",
//             "document:2#owner@beth",
//             "document:2#viewer@organization:contoso#member",
//         ];
//
//         for rel_str in relationships.iter() {
//             let relationship = rel_str.parse::<Relationship>().unwrap();
//             graph.add_relation(
//                 relationship.subject.to_string(),
//                 relationship.relation,
//                 relationship.object.to_string(),
//             );
//         }
//
//         // graph.initialize_group_hierarchies();
//         graph
//     }
//
//     #[test]
//     fn test_permissions_for_owner() {
//         let graph = setup_document_graph();
//         let subject = Subject::User("anne".to_string());
//         let object1 = "document:1".parse::<Object>().unwrap();
//
//         // print dot
//         println!("{}", graph.dot());
//         println!("{}", graph.mermaid());
//
//         // Checks if Anne, as the owner, has permission to share the document
//         assert!(
//             graph.check_access(&subject, "can_share", &object1),
//             "Owner should be able to share"
//         );
//
//         // Checks if Anne, as the owner, has permission to write to the document
//         assert!(
//             graph.check_access(&subject, "can_write", &object1),
//             "Owner should be able to write"
//         );
//
//         // Checks if Anne, as the owner, has permission to view the document
//         assert!(
//             graph.check_access(&subject, "can_view", &object1,),
//             "Owner should be able to view"
//         );
//
//         // Checks if Anne, as the owner, has permission to change the owner of the document
//         assert!(
//             graph.check_access(&subject, "can_change_owner", &object1),
//             "Owner should be able to change ownership"
//         );
//     }
//
//     #[test]
//     fn test_permissions_for_editor_via_group() {
//         let graph = setup_document_graph();
//         let subject = Subject::User("beth".to_string());
//         let object1 = "document:1".parse::<Object>().unwrap();
//
//         // Checks if Beth, as an editor through group membership, has permission to share the document
//         assert!(
//             graph.check_access(&subject, "can_share", &object1),
//             "Editor should be able to share"
//         );
//
//         // Checks if Beth, as an editor through group membership, has permission to write to the document
//         assert!(
//             graph.check_access(&subject, "can_write", &object1),
//             "Editor should be able to write"
//         );
//
//         // Checks if Beth, as an editor through group membership, has permission to view the document
//         assert!(
//             graph.check_access(&subject, "can_view", &object1),
//             "Editor should be able to view"
//         );
//
//         // Verifies that Beth, despite being an editor, cannot change the ownership of the document
//         assert!(
//             !graph.check_access(&subject, "can_change_owner", &object1),
//             "Editor should not be able to change ownership"
//         );
//     }
//
//     #[test]
//     fn test_viewer_permissions_from_group_membership() {
//         let graph = setup_document_graph();
//         let subject = Subject::User("anne".to_string());
//         let object2 = "document:2".parse::<Object>().unwrap();
//
//         // Anne, through group membership, should not have permission to share document:2
//         assert!(
//             !graph.check_access(&subject, "can_share", &object2),
//             "Viewer should not be able to share"
//         );
//
//         // Anne, through group membership, should not have permission to write to document:2
//         assert!(
//             !graph.check_access(&subject, "can_write", &object2),
//             "Viewer should not be able to write"
//         );
//
//         // Anne, through group membership, should have permission to view document:2
//         assert!(
//             graph.check_access(&subject, "can_view", &object2),
//             "Viewer should be able to view"
//         );
//
//         // Anne, through group membership, should not have permission to change the ownership of document:2
//         assert!(
//             !graph.check_access(&subject, "can_change_owner", &object2),
//             "Viewer should not be able to change ownership"
//         );
//     }
//
//     #[test]
//     fn test_negative_case_for_non_member() {
//         let graph = setup_document_graph();
//         let subject = Subject::User("charlie".to_string());
//         let object1 = "document:1".parse::<Object>().unwrap();
//         let object2 = "document:2".parse::<Object>().unwrap();
//
//         // Charlie has no defined relationships with any documents, should not view document:1
//         assert!(
//             !graph.check_access(&subject, "can_view", &object1),
//             "Non-member should not be able to view"
//         );
//
//         // Charlie has no defined relationships with any documents, should not share document:2
//         assert!(
//             !graph.check_access(&subject, "can_share", &object2),
//             "Non-member should not be able to share"
//         );
//
//         // Charlie has no defined relationships with any documents, should not change ownership of document:1
//         assert!(
//             !graph.check_access(&subject, "can_change_owner", &object1),
//             "Non-member should not be able to change ownership"
//         );
//     }
//
//     #[test]
//     fn test_cache_effectiveness_after_update() {
//         let mut graph = setup_document_graph();
//         let subject = "anne".parse::<Subject>().unwrap();
//         let object1 = "document:1".parse::<Object>().unwrap();
//
//         // First check to populate the cache. This is to ensure that subsequent checks are faster
//         // and that the cache is being used properly. We expect Anne to have view access to document:1.
//         assert!(
//             graph.check_access(&subject, "can_view", &object1),
//             "Anne should initially have view access to document:1"
//         );
//
//         // After the initial access check, we now verify that the cache has recorded the access.
//         // This is done to ensure that the data related to permissions is stored and reused.
//         let cache_hit = graph.check_cache(&subject, &object1, "can_view");
//         assert!(
//             cache_hit.is_some() && cache_hit.unwrap(),
//             "Cache should have a hit for Anne's view access to document:1"
//         );
//
//         // Updating the relationship to simulate a change which might affect caching.
//         // Here we reinforce Anne's access by explicitly stating it again as a viewer. This step is typically used
//         // to test if the cache is invalidated or updated correctly when underlying data changes.
//         let new_relation = "document:1#viewer@anne".parse::<Relationship>().unwrap();
//         graph.add_relation(
//             new_relation.subject.to_string(),
//             new_relation.relation,
//             new_relation.object.to_string(),
//         );
//
//         // Check access again to see if cache has been updated or if it still returns the correct result.
//         // The expectation here is that Anne still has view access, but we're also checking if the cache has handled
//         // the relationship update correctly, either by invalidating the outdated entry or updating it.
//         assert!(
//             graph.check_access(&subject, "can_view", &object1),
//             "Anne should still have view access to document:1 after updating relationship"
//         );
//     }
// }
