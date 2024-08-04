// use std::collections::HashSet;
// use std::hash::Hash;
//
// use petgraph::graph::{DiGraph, NodeIndex};
// use petgraph::prelude::EdgeRef;
//
// use crate::graph::relation_graph::{Node, Relation};
//
// // Recursive helper function to explore all descendant groups in a generic fashion
// fn explore_subgroups<U, G, O>(
//     graph: &DiGraph<Node<U, G, O>, Relation>, group_index: NodeIndex,
//     visited: &mut HashSet<NodeIndex>, subgroups: &mut Vec<G>,
// ) where
//     U: Eq + Hash + Clone + std::fmt::Display,
//     G: Eq + Hash + Clone + std::fmt::Display,
//     O: Eq + Hash + Clone + std::fmt::Display,
// {
//     // Mark the current group as visited
//     if !visited.insert(group_index) {
//         return; // If already visited, then return to avoid infinite loops
//     }
//
//     // Explore all connected nodes
//     for edge in graph.edges_directed(group_index, petgraph::Direction::Outgoing) {
//         let target_index = edge.target();
//         // Check if the target node is a group and not yet visited
//         if let Some(crate::graph::relation_graph::Node::Group(subgroup)) =
//             graph.node_weight(target_index)
//         {
//             subgroups.push(subgroup.clone()); // Add the subgroup to the list
//             explore_subgroups(graph, target_index, visited, subgroups); // Recursively explore further
//         }
//     }
// }
