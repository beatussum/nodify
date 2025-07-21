//! Many different processes
//!
//! This module contains several different [`Process`es](Process)

use crate::Node;

pub mod dfs;
pub use dfs::DFS;

#[cfg(feature = "rayon")]
pub mod parallel_dfs;

#[cfg(feature = "rayon")]
pub use parallel_dfs::ParallelDFS;

/// A [`Process`] allowing to apply some transformations to a [`Node`]
pub trait Process {
    /// The underlying [`Node`] type of this [`Process`]
    type Node: Node;

    /// Build a [`Process`] from a [`Node`]
    fn from_node(node: Self::Node) -> Self;
}

/// A [`Process`] allowing to check a graph contains any [`Node`] verifying a
/// given predicate
pub trait Contains<I, P>: Process
where
    P: Fn(I) -> bool,
{
    /// Check if a graph contains any [`Node`] verifying the given predicate
    /// _pred_
    fn contains(&self, pred: P) -> bool;
}

/// A [`Process`] allowing to find any [`Node`] verifying a given predicate
pub trait FindAny<I, P>: Process
where
    P: Fn(I) -> bool,
{
    /// Search for some item that matches with the given predicate
    ///
    /// This operation is similar to [`find()`](Iterator::find) but the item
    /// returned may not be the **first** one.
    ///
    /// For a weighted graph, the _first_ element is the one with the lowest
    /// distance from the start node.
    fn find_any(&self, pred: P) -> Option<Self::Node>;
}

impl<I, FA, P> Contains<I, P> for FA
where
    FA: FindAny<I, P>,
    P: Fn(I) -> bool,
{
    fn contains(&self, pred: P) -> bool {
        self.find_any(pred).is_some()
    }
}
