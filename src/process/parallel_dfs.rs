//! This module contains the implementation of [`ParallelDFS`]

use super::{ContainsAny, Process};
use crate::Node;
use std::{collections::LinkedList, hash::Hash};

type HashSet<K> = dashmap::DashSet<K, ahash::RandomState>;

/// A parallel [DFS](https://en.wikipedia.org/wiki/Depth-first_search) implementation of some processes
///
/// In particular, the following [`Process`es](Process) are implemented:
/// - [`ContainsAny`].
pub struct ParallelDFS<N> {
    node: N,
}

impl<N> Process for ParallelDFS<N>
where
    N: Node,
{
    type Node = N;

    fn from_node(node: Self::Node) -> Self {
        Self { node }
    }
}

impl<N, P> ContainsAny<P> for ParallelDFS<N>
where
    N: Copy + Eq + Hash + Node + Send + Sync,
    P: Fn(Self::Node) -> bool + Sync,
{
    fn contains_any(&self, pred: P) -> bool {
        use rayon::prelude::*;

        fn contains_any_until<N, P>(
            is_visited: &HashSet<N>,
            mut to_visit: Vec<N>,
            threshold: usize,
            pred: &P,
        ) -> Option<Vec<N>>
        where
            N: Copy + Eq + Hash + Node,
            P: Fn(N) -> bool,
        {
            for _ in 0..threshold {
                match to_visit.pop() {
                    None => break,

                    Some(node) => {
                        if is_visited.insert(node) {
                            let next = node.outgoing().filter(|node| !is_visited.contains(node));

                            for node in next {
                                if pred(node) {
                                    return None;
                                } else {
                                    to_visit.push(node);
                                }
                            }
                        }
                    }
                }
            }

            Some(to_visit)
        }

        let max_task = rayon::current_num_threads();
        let threshold = 50_000;

        let is_visited = HashSet::default();
        let mut to_visit = vec![self.node];

        while !to_visit.is_empty() {
            let len = to_visit.len();

            if len < max_task {
                let next = contains_any_until(&is_visited, to_visit, threshold, &pred);

                match next {
                    Some(next) => to_visit = next,
                    None => return true,
                }
            } else {
                let next = to_visit
                    .par_drain(len.saturating_sub(max_task)..)
                    .chunks(1)
                    .try_fold(LinkedList::new, |mut next, to_visit| {
                        let to_push = contains_any_until(&is_visited, to_visit, threshold, &pred)?;
                        next.push_back(to_push);
                        Some(next)
                    })
                    .try_reduce(LinkedList::new, |mut lhs, mut rhs| {
                        lhs.append(&mut rhs);
                        Some(lhs)
                    })
                    .map(|list| {
                        list.into_iter()
                            .reduce(|mut lhs, mut rhs| {
                                lhs.append(&mut rhs);
                                lhs
                            })
                            .unwrap_or_default()
                    });

                match next {
                    Some(mut next) => to_visit.append(&mut next),
                    None => return true,
                }
            }
        }

        false
    }
}
