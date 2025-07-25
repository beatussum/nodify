//! This module contains the implementation of [`ParallelDFS`]

use super::{Contains, FindAny, Process};
use crate::{Node, ToValue};
use std::{collections::LinkedList, hash::Hash};

type HashSet<K> = dashmap::DashSet<K, ahash::RandomState>;

/// A parallel [DFS](https://en.wikipedia.org/wiki/Depth-first_search) implementation of some processes
///
/// In particular, the following [`Process`es](Process) are implemented:
/// - [`Contains`],
/// - [`FindAny`].
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct ParallelDFS<N> {
    node: N,
}

impl<N> Process for ParallelDFS<N> {
    type Node = N;

    fn from_node(node: Self::Node) -> Self {
        Self { node }
    }
}

impl<I, N, P> Contains<I, P> for ParallelDFS<N>
where
    N: Copy + Eq + Hash + Node + Send + Sync + ToValue<I>,
    P: Fn(I) -> bool + Sync,
{
    fn contains(&self, pred: P) -> bool {
        self.find_any(pred).is_some()
    }
}

impl<I, N, P> FindAny<I, P> for ParallelDFS<N>
where
    N: Copy + Eq + Hash + Node + Send + Sync + ToValue<I>,
    P: Fn(I) -> bool + Sync,
{
    fn find_any(&self, pred: P) -> Option<Self::Node> {
        use rayon::prelude::*;

        fn next_until<I, N, P>(
            is_visited: &HashSet<N>,
            mut to_visit: Vec<N>,
            threshold: usize,
            pred: &P,
        ) -> Result<Vec<N>, N>
        where
            N: Copy + Eq + Hash + ToValue<I> + Node,
            P: Fn(I) -> bool,
        {
            for _ in 0..threshold {
                match to_visit.pop() {
                    None => break,

                    Some(node) => {
                        if is_visited.insert(node) {
                            let next = node.outgoing().filter(|node| !is_visited.contains(node));

                            for node in next {
                                if pred(node.to_value()) {
                                    return Err(node);
                                } else {
                                    to_visit.push(node);
                                }
                            }
                        }
                    }
                }
            }

            Ok(to_visit)
        }

        let max_task = rayon::current_num_threads();
        let threshold = 50_000;

        let is_visited = HashSet::default();
        let mut to_visit = vec![self.node];

        while !to_visit.is_empty() {
            let len = to_visit.len();

            if len < max_task {
                let next = next_until(&is_visited, to_visit, threshold, &pred);

                match next {
                    Ok(next) => to_visit = next,
                    Err(ret) => return Some(ret),
                }
            } else {
                let next = to_visit
                    .par_drain(len.saturating_sub(max_task)..)
                    .chunks(1)
                    .try_fold(LinkedList::new, |mut next, to_visit| {
                        let to_push = next_until(&is_visited, to_visit, threshold, &pred)?;
                        next.push_back(to_push);
                        Ok(next)
                    })
                    .try_reduce(LinkedList::new, |mut lhs, mut rhs| {
                        lhs.append(&mut rhs);
                        Ok(lhs)
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
                    Ok(mut next) => to_visit.append(&mut next),
                    Err(ret) => return Some(ret),
                }
            }
        }

        None
    }
}
