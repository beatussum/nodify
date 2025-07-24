//! This module contains the implementation of [`DeltaStepping`]

use super::{Contains, FindAny, FindFirst, Process};
use crate::{ToValue, Weighted};
use num_traits::Unsigned;
use rayon::prelude::*;
use std::hash::Hash;

type HashMap<K, V> = dashmap::DashMap<K, V, ahash::RandomState>;
type HashMultiMap<K, V> = HashMap<K, Vec<V>>;

/// A [delta stepping algorithm](https://en.wikipedia.org/wiki/Parallel_single-source_shortest_path_algorithm#Delta_stepping_algorithm)
/// implementation of some [`Process`es](Process).
///
/// In particular, the following [`Process`es](Process) are implemented:
/// - [`Contains`],
/// - [`FindAny`],
/// - [`FindFirst`].
pub struct DeltaStepping<N, W> {
    base: N,
    delta: W,
    buckets: HashMultiMap<W, N>,
    dists: HashMap<N, W>,
}

impl<N, W> DeltaStepping<N, W>
where
    N: Send + Sync,
    W: Copy + Eq + Hash + Ord + Send + Sync,
{
    fn first_bucket_index(&self) -> Option<W> {
        self.buckets
            .par_iter_mut()
            .filter(|r| !r.value().is_empty())
            .map(|r| *r.key())
            .min()
    }
}

impl<N, W: Copy> DeltaStepping<N, W> {
    fn node<'a>(&'a self, node: N) -> DeltaSteppingNode<'a, N, W> {
        DeltaSteppingNode {
            node,
            delta: self.delta,
            buckets: &self.buckets,
            dists: &self.dists,
        }
    }
}

impl<N, W> DeltaStepping<N, W> {
    /// Consumme the current [`DeltaStepping`] instance and create another with
    /// the same values as before except for the value of delta which is updated
    /// to `delta`.
    ///
    /// # Example
    ///
    /// You can consult the complete version of this example at
    /// `examples/knapsack.rs`.
    ///
    /// ```
    /// # use nodify::prelude::*;
    /// # use std::iter::once;
    /// #
    /// # #[derive(Clone, Copy, Hash, PartialEq, Eq)]
    /// # pub struct Item {
    /// #     pub value: u32,
    /// #     pub weight: u32,
    /// # }
    /// #
    /// # #[derive(Clone, Copy, Hash, PartialEq, Eq)]
    /// # pub struct Knapsack<'a> {
    /// #     capacity: u32,
    /// #     items: &'a [Item],
    /// #     value: u32,
    /// #     max_value: u32,
    /// # }
    /// #
    /// # impl<'a> Knapsack<'a> {
    /// #     pub fn new(capacity: u32, items: &'a [Item]) -> Option<Self> {
    /// #         items.iter().max_by_key(|Item { value, .. }| value).map(
    /// #             |&Item {
    /// #                  value: max_value, ..
    /// #              }| {
    /// #                 Self {
    /// #                     capacity,
    /// #                     items,
    /// #                     value: 0,
    /// #                     max_value,
    /// #                 }
    /// #             },
    /// #         )
    /// #     }
    /// #
    /// #     pub fn is_solution(&self) -> bool {
    /// #         self.items.is_empty()
    /// #     }
    /// # }
    /// #
    /// # impl Weighted for Knapsack<'_> {
    /// #     type Weight = u32;
    /// #
    /// #     fn weighted_outgoing(self) -> impl Iterator<Item = (Self::Weight, Self)> {
    /// #         self.items
    /// #             .split_first()
    /// #             .map(|(&Item { weight, value }, items)| {
    /// #                 once({
    /// #                     let node = Self {
    /// #                         capacity: self.capacity,
    /// #                         items,
    /// #                         value: self.value,
    /// #                         max_value: self.max_value,
    /// #                     };
    /// #
    /// #                     (self.max_value, node)
    /// #                 })
    /// #                 .chain((weight <= self.capacity).then(|| {
    /// #                     let node = Knapsack {
    /// #                         capacity: self.capacity - weight,
    /// #                         items,
    /// #                         value: self.value + value,
    /// #                         max_value: self.max_value,
    /// #                     };
    /// #
    /// #                     (self.max_value - value, node)
    /// #                 }))
    /// #             })
    /// #             .into_iter()
    /// #             .flatten()
    /// #     }
    /// # }
    /// #
    /// # impl Node for Knapsack<'_> {
    /// #     fn outgoing(self) -> impl Iterator<Item = Self> {
    /// #         self.weighted_outgoing().map(|(_, outgoing)| outgoing)
    /// #     }
    /// # }
    /// #
    /// # fn main() -> Result<(), &'static str> {
    /// #     let items = [
    /// #         Item {
    /// #             value: 1,
    /// #             weight: 1,
    /// #         },
    /// #         Item {
    /// #             value: 7,
    /// #             weight: 2,
    /// #         },
    /// #         Item {
    /// #             value: 11,
    /// #             weight: 3,
    /// #         },
    /// #     ];
    /// #
    /// #     let capacity = 5;
    /// #     let root = Knapsack::new(capacity, &items).ok_or("Root creation failed")?;
    /// #
    ///     let Knapsack {
    ///         capacity: mut weight,
    ///         value,
    ///         ..
    ///     } = root
    ///         .to_process::<DeltaStepping<_, _>>()
    ///         .with_delta(2)
    ///         .find_first(|node| node.is_solution())
    ///         .ok_or("No solution")?;
    /// #
    /// #     weight = capacity - weight;
    /// #
    /// #     println!("value = {value}, weight = {weight}");
    /// #
    /// #     Ok(())
    /// # }
    /// ```
    pub fn with_delta(self, delta: W) -> Self {
        Self {
            base: self.base,
            delta,
            buckets: self.buckets,
            dists: self.dists,
        }
    }
}

impl<N, W> Process for DeltaStepping<N, W>
where
    N: Copy + Eq + Hash,
    W: Default + Eq + Hash + Unsigned,
{
    type Node = N;

    fn from_node(node: Self::Node) -> Self {
        let delta = W::default();
        let buckets = HashMultiMap::from_iter([(W::zero(), vec![node])]);
        let dists = HashMap::from_iter([(node, W::zero())]);

        Self {
            base: node,
            delta,
            buckets,
            dists,
        }
    }
}

impl<I, N, P, W> Contains<I, P> for DeltaStepping<N, W>
where
    N: ToValue<I> + Copy + Eq + Hash + Send + Sync + Weighted<Weight = W>,
    P: Fn(I) -> bool + Sync,
    W: Copy + Default + Eq + Hash + Ord + Send + Sync + Unsigned,
{
    fn contains(&self, pred: P) -> bool {
        self.find_first(pred).is_some()
    }
}

impl<I, N, P, W> FindAny<I, P> for DeltaStepping<N, W>
where
    N: ToValue<I> + Copy + Eq + Hash + Send + Sync + Weighted<Weight = W>,
    P: Fn(I) -> bool + Sync,
    W: Copy + Default + Eq + Hash + Ord + Send + Sync + Unsigned,
{
    fn find_any(&self, pred: P) -> Option<Self::Node> {
        self.find_first(pred)
    }
}

impl<I, N, P, W> FindFirst<I, P> for DeltaStepping<N, W>
where
    N: ToValue<I> + Copy + Eq + Hash + Send + Sync + Weighted<Weight = W>,
    P: Fn(I) -> bool + Sync,
    W: Copy + Default + Eq + Hash + Ord + Send + Sync + Unsigned,
{
    fn find_first(&self, pred: P) -> Option<Self::Node> {
        use std::collections::LinkedList;

        while let Some(first_index) = self.first_bucket_index() {
            let mut heavy_edges = LinkedList::default();

            while let Some((_, first_bucket)) = self.buckets.remove(&first_index) {
                let mut to_append = first_bucket
                    .into_par_iter()
                    .fold(LinkedList::new, |mut list, node| {
                        let to_push = self.node(node).explore();
                        list.push_back(to_push);
                        list
                    })
                    .reduce(LinkedList::new, |mut lhs, mut rhs| {
                        lhs.append(&mut rhs);
                        lhs
                    });

                heavy_edges.append(&mut to_append);
            }

            heavy_edges
                .into_par_iter()
                .flatten()
                .for_each(|(new_dist, node)| self.node(node).relax(new_dist));
        }

        self.dists
            .par_iter()
            .map(|r| {
                let (&node, &dist) = r.pair();
                (node, dist)
            })
            .filter(|&(node, _)| pred(node.to_value()))
            .min_by_key(|&(_, dist)| dist)
            .map(|(node, _)| node)
    }
}

struct DeltaSteppingNode<'a, N, W> {
    node: N,
    delta: W,
    buckets: &'a HashMultiMap<W, N>,
    dists: &'a HashMap<N, W>,
}

impl<N, W> DeltaSteppingNode<'_, N, W>
where
    N: Copy + Eq + Hash + Weighted<Weight = W>,
    W: Copy + Hash + Ord + Unsigned,
{
    fn explore(self) -> Vec<(W, N)> {
        let Self {
            node,
            delta,
            buckets,
            dists,
        } = self;

        let base_dist = self
            .dists
            .get(&self.node)
            .as_deref()
            .copied()
            .unwrap_or_else(W::zero);

        let mut heavy_edges = Vec::default();

        for (new_dist, node) in node
            .weighted_outgoing()
            .map(|(w, node)| (base_dist + w, node))
        {
            if new_dist > delta {
                heavy_edges.push((new_dist, node));
            } else {
                Self {
                    node,
                    delta: self.delta,
                    buckets,
                    dists,
                }
                .relax(new_dist);
            }
        }

        heavy_edges
    }
}

impl<N, W> DeltaSteppingNode<'_, N, W>
where
    N: Copy + Eq + Hash,
    W: Copy + Hash + Ord + Unsigned,
{
    fn relax(self, new_dist: W) {
        let to_insert = self
            .dists
            .get(&self.node)
            .as_deref()
            .copied()
            .map_or(Some(new_dist), |old_dist| {
                (new_dist < old_dist).then_some(new_dist)
            });

        if let Some(new_dist) = to_insert {
            self.dists.insert(self.node, new_dist);

            self.buckets
                .entry(new_dist / self.delta)
                .or_default()
                .push(self.node);
        }
    }
}
