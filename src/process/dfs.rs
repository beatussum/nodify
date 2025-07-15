//! This module contains the implementation of [`DFS`]

use super::{ContainsAny, Process};
use crate::Node;
use std::hash::Hash;

/// A [DFS](https://en.wikipedia.org/wiki/Depth-first_search) implementation of some processes
///
/// In particular, the following [`Process`es](Process) are implemented:
/// - [`ContainsAny`].
pub struct DFS<N> {
    node: N,
}

impl<N: Node> Process for DFS<N> {
    type Node = N;

    fn from_node(node: Self::Node) -> Self {
        Self { node }
    }
}

impl<N, P> ContainsAny<P> for DFS<N>
where
    N: Copy + Eq + Hash + Node,
    P: Fn(N::Value) -> bool,
{
    fn contains_any(&self, pred: P) -> bool {
        type HashSet<K> = std::collections::HashSet<K, ahash::RandomState>;

        let mut is_visited = HashSet::default();
        let mut to_visit = vec![self.node];

        while let Some(node) = to_visit.pop() {
            if pred(node.value()) {
                return true;
            } else if is_visited.insert(node) {
                let next = node.outgoing().filter(|node| !is_visited.contains(node));
                to_visit.extend(next);
            }
        }

        false
    }
}
