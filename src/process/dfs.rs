//! This module contains the implementation of [`DFS`]

use super::{FindAny, Process};
use crate::{AsValue, Node};
use std::hash::Hash;

/// A [DFS](https://en.wikipedia.org/wiki/Depth-first_search) implementation of some processes
///
/// In particular, the following [`Process`es](Process) are implemented:
/// - [`FindAny`].
pub struct DFS<N> {
    node: N,
}

impl<N: Node> Process for DFS<N> {
    type Node = N;

    fn from_node(node: Self::Node) -> Self {
        Self { node }
    }
}

impl<I, N, P> FindAny<I, P> for DFS<N>
where
    N: Copy + Eq + Hash + AsValue<I> + Node,
    P: Fn(I) -> bool,
{
    fn find_any(&self, pred: P) -> Option<Self::Node> {
        type HashSet<K> = std::collections::HashSet<K, ahash::RandomState>;

        let mut is_visited = HashSet::default();
        let mut to_visit = vec![self.node];

        while let Some(node) = to_visit.pop() {
            if pred(node.as_value()) {
                return Some(node);
            } else if is_visited.insert(node) {
                let next = node.outgoing().filter(|node| !is_visited.contains(node));
                to_visit.extend(next);
            }
        }

        None
    }
}
