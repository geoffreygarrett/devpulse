// use std::collections::{HashMap, HashSet};
// use std::fmt::Debug;
// use std::hash::Hash;
// use std::sync::Arc;
//
// use derive_more::From;
// use skiplist::SkipList;
// use tokio::sync::RwLock as AsyncRwLock;
// use tokio::time::Instant;
// use tracing::{error, info};
//
// // Enum representing the set type
// #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
// enum SetType {
//     Group2Group,
//     Member2Group,
// }
//
// type SetId = u64;
// type ElementId = u64;
//
// // Struct representing an index entry in the Leopard system
// #[derive(Debug, Clone)]
// struct RootIndexEntry<S, E> {
//     set_type: SetType,
//     set_id: S,
//     element_id: E,
// }
//
// impl<S, E> RootIndexEntry<S, E> {
//     fn new(set_type: SetType, set_id: S, element_id: E) -> Self {
//         Self {
//             set_type,
//             set_id,
//             element_id,
//         }
//     }
// }
//
// // SkipList for efficient operations
// struct RootIndex<S, E>(SkipList<RootIndexEntry<S, E>>);
//
// impl<S, E> RootIndex<S, E>
// where
//     S: Eq + Hash + Clone + Debug,
//     E: Eq + Hash + Clone + Debug,
// {
//     fn new() -> Self {
//         RootIndex(HashMap::new())
//     }
//
//     fn merge_diff_index(&mut self, diff_index: &DiffIndex<S, E>) {
//         for entry in diff_index.iter() {
//             let IndexEntry {
//                 set_id, element_id, ..
//             } = entry.index;
//             if entry.deletion_marker {
//                 if let Some(elements) = self.0.get_mut(&set_id) {
//                     elements.remove(&element_id);
//                     if elements.is_empty() {
//                         self.0.remove(&set_id);
//                     }
//                 }
//             } else {
//                 self.0
//                     .entry(set_id.clone())
//                     .or_insert_with(HashSet::new)
//                     .insert(element_id.clone());
//             }
//         }
//     }
// }
//
// /// Represents an incremental index in the Leopard system.
// ///
// /// (T, s, e, t, d) tuple, where t is the timestamp of the update and d is a deletion marker.
// #[derive(Debug)]
// struct DiffIndexEntry<S, E, T = Instant> {
//     index: RootIndexEntry<S, E>,
//     timestamp: T,
//     deletion_marker: bool,
// }
//
// impl<S, E, T> DiffIndexEntry<S, E, T> {
//     fn new(
//         set_type: SetType, set_id: S, element_id: E, timestamp: T, deletion_marker: bool,
//     ) -> Self {
//         Self {
//             index: RootIndexEntry::new(set_type, set_id, element_id),
//             timestamp,
//             deletion_marker,
//         }
//     }
// }
//
// struct DiffIndex<S, E, T = Instant>(SkipList<DiffIndexEntry<S, E, T>>);
//
// // // models.rs
// // #[derive(Debug, Clone, PartialEq, Eq, Hash, From)]
// // pub struct Identifier(String); // Binary representation of identifiers
//
// /// Represents an indexing system for group membership based on the principles outlined in Zanzibar.
// ///
// /// This struct manages both group-to-group and member-to-group relationships.
// /// The indices are designed to facilitate efficient set computations to determine group memberships.
// ///
// /// # Type Parameters
// /// - `S`: Type representing the set identifier.
// /// - `E`: Type representing the element identifier.
// ///
// /// # References
// ///
// /// - [Zanzibar: Google's Consistent, Global Authorization System](https://research.google/pubs/pub48190/)
// ///   See section 3.2.4 Leopard Indexing System for details on the indexing strategy used here.
// ///
// #[derive(Debug)]
// pub(crate) struct Leopard<S, E>
// where
//     S: std::hash::Hash + Eq + Clone,
//     E: std::hash::Hash + Eq + Clone,
// {
//     root_index: Arc<AsyncRwLock<RootIndex<S, E>>>,
//     diff_index: Arc<AsyncRwLock<DiffIndex<S, E>>>,
// }
//
// /// Implementation of the `GroupIndex`.
// impl<U, G> Leopard<U, G>
// where
//     U: std::hash::Hash + Eq + Clone + Debug,
//     G: std::hash::Hash + Eq + Clone + Debug,
// {
//     /// Constructs a new `GroupIndex`.
//     pub fn new() -> Self {
//         Self {
//             root_index: Arc::new(AsyncRwLock::new(SkipList::new())),
//             diff_index: Arc::new(AsyncRwLock::new(SkipList::new())),
//         }
//     }
//
//     /// Asynchronously updates the group-to-group relationships in the index.
//     pub async fn update_group_to_group(&self, group: &'g G, subgroups: HashSet<&'g G>) {
//         let mut write_guard = self.group_to_group.write().await;
//         write_guard.insert(&group, subgroups);
//         info!("Updated group-to-group relationships for {:?}", group);
//     }
//
//     /// Asynchronously updates the member-to-group relationships in the index.
//     pub async fn update_member_to_group(&self, user: &'g U, groups: HashSet<&'g G>) {
//         let mut write_guard = self.member_to_group.write().await;
//         write_guard.insert(user, groups);
//         info!("Updated member-to-group relationships for {:?}", user);
//     }
//
//     /// Asynchronously checks if a user is a member of a specified group, considering both direct and indirect memberships.
//     pub async fn is_member_of_group(&self, user: &'g U, group: &'g G) -> bool {
//         let member_groups = self.member_to_group.read().await;
//         let group_groups = self.group_to_group.read().await;
//
//         if let (Some(user_groups), Some(target_groups)) =
//             (member_groups.get(user), group_groups.get(group))
//         {
//             let is_member = !user_groups.is_disjoint(target_groups);
//             info!("User {:?} is_member_of_group {:?}: {}", user, group, is_member);
//             is_member
//         } else {
//             error!("Group or user not found in indices");
//             false
//         }
//     }
// }
//
// // n, Leopard servers maintain an
// // incremental layer that indexes all updates since the offline
// // snapshot, where each update is represented by a (T,s, e,t,d)
// // tuple, where t is the timestamp of the update and d is a deletion marker. Updates with timestamps less than or equal to
// // the query timestamp are merged on top of the offline index

use std::fmt::Debug;
use std::hash::Hash;

use derive_more::From;
use skiplist::SkipList;
use tokio::time::Instant;

// Enum representing the set type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum SetType {
    Group2Group,
    Member2Group,
}

type SetId = u64;
type ElementId = u64;

// Struct representing an index entry in the Leopard system
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct RootIndexEntry<S, E> {
    set_type: SetType,
    set_id: S,
    element_id: E,
}

impl<S, E> RootIndexEntry<S, E> {
    fn new(set_type: SetType, set_id: S, element_id: E) -> Self {
        Self {
            set_type,
            set_id,
            element_id,
        }
    }
}

// Generic trait for index operations
trait IndexOps<S, E> {
    fn insert(&mut self, entry: RootIndexEntry<S, E>);
    fn remove(&mut self, entry: &RootIndexEntry<S, E>);
    fn merge_diff_index(&mut self, diff_index: &DiffIndex<S, E>);
}

// SkipList-based implementation of IndexOps
#[derive(Debug)]
struct SkipListIndex<S, E>(SkipList<RootIndexEntry<S, E>>);

impl<S, E> std::fmt::Display for SkipListIndex<S, E>
where
    S: Debug,
    E: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.0)
    }
}

impl<S, E> SkipListIndex<S, E> {
    fn new() -> Self {
        SkipListIndex(SkipList::new())
    }
}

impl<S, E> IndexOps<S, E> for SkipListIndex<S, E>
where
    S: Eq + Hash + Clone + Debug,
    E: Eq + Hash + Clone + Debug,
{
    fn insert(&mut self, entry: RootIndexEntry<S, E>) {
        self.0.push_back(entry);
    }

    fn remove(&mut self, entry: &RootIndexEntry<S, E>) {
        let index = self.0.iter().position(|x| x == entry);
        if let Some(index) = index {
            self.0.remove(index);
        }
    }

    fn merge_diff_index(&mut self, diff_index: &DiffIndex<S, E>) {
        for entry in diff_index.0.iter() {
            if entry.deletion_marker {
                self.remove(&entry.index);
            } else {
                self.insert(entry.index.clone());
            }
        }
    }
}

// Represents an incremental index in the Leopard system.
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct DiffIndexEntry<S, E, T = Instant> {
    index: RootIndexEntry<S, E>,
    timestamp: T,
    deletion_marker: bool,
}

impl<S, E, T> DiffIndexEntry<S, E, T> {
    fn new(
        set_type: SetType, set_id: S, element_id: E, timestamp: T, deletion_marker: bool,
    ) -> Self {
        Self {
            index: RootIndexEntry::new(set_type, set_id, element_id),
            timestamp,
            deletion_marker,
        }
    }
}

struct DiffIndex<S, E, T = Instant>(SkipList<DiffIndexEntry<S, E, T>>);

impl<S, E, T> DiffIndex<S, E, T> {
    fn new() -> Self {
        DiffIndex(SkipList::new())
    }
}

// Leopard struct with generic index implementation
struct Leopard<S, E, I>
where
    I: IndexOps<S, E>,
{
    root_index: I,
    diff_index: DiffIndex<S, E>,
}

impl<S, E, I> Leopard<S, E, I>
where
    I: IndexOps<S, E>,
{
    fn new(index: I) -> Self {
        Self {
            root_index: index,
            diff_index: DiffIndex::new(),
        }
    }

    fn merge_diff_index(&mut self) {
        self.root_index.merge_diff_index(&self.diff_index);
    }
}

// Tests
#[cfg(test)]
mod tests {
    use tokio::time::Instant;

    use super::*;

    #[test]
    fn test_skiplist_index_insert_and_remove() {
        let mut index = SkipListIndex::<SetId, ElementId>::new();

        let entry = RootIndexEntry::new(SetType::Group2Group, 1, 100);
        index.insert(entry.clone());
        assert!(index.0.contains(&entry));

        index.remove(&entry);
        assert!(!index.0.contains(&entry));
    }

    #[test]
    fn test_merge_diff_index() {
        let mut root_index = SkipListIndex::<SetId, ElementId>::new();
        let mut diff_index = DiffIndex::<SetId, ElementId>::new();

        let entry1 = DiffIndexEntry::new(SetType::Group2Group, 1, 100, Instant::now(), false);
        let entry2 = DiffIndexEntry::new(SetType::Group2Group, 1, 101, Instant::now(), true);

        diff_index.0.push_back(entry1.clone());
        diff_index.0.push_back(entry2.clone());

        root_index.insert(entry1.index.clone());
        assert!(root_index.0.contains(&entry1.index));

        root_index.merge_diff_index(&diff_index);
        assert!(root_index.0.contains(&entry1.index));
        assert!(!root_index.0.contains(&entry2.index));
    }

    #[test]
    fn test_leopard_merge_diff_index() {
        let root_index = SkipListIndex::<SetId, ElementId>::new();
        let mut leopard = Leopard::new(root_index);

        let entry1 = DiffIndexEntry::new(SetType::Group2Group, 1, 100, Instant::now(), false);
        let entry2 = DiffIndexEntry::new(SetType::Group2Group, 1, 101, Instant::now(), true);

        leopard.diff_index.0.push_back(entry1.clone());
        leopard.diff_index.0.push_back(entry2.clone());

        leopard.merge_diff_index();
        assert!(leopard.root_index.0.contains(&entry1.index));
        assert!(!leopard.root_index.0.contains(&entry2.index));
    }
}

fn main() {
    let root_index = SkipListIndex::new();
    let mut leopard = Leopard::new(root_index);

    let entry1 = DiffIndexEntry::new(SetType::Group2Group, 1, 100, Instant::now(), false);
    let entry2 = DiffIndexEntry::new(SetType::Group2Group, 1, 101, Instant::now(), true);

    leopard.diff_index.0.push_back(entry1);
    leopard.diff_index.0.push_back(entry2);

    leopard.merge_diff_index();

    println!("{:?}", leopard.root_index);
}
