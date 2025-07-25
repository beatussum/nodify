//! Many different processes
//!
//! This module contains several different [`Process`es](Process)

pub mod dfs;
pub use dfs::DFS;

#[cfg(feature = "rayon")]
pub mod parallel_dfs;

#[cfg(feature = "rayon")]
pub use parallel_dfs::ParallelDFS;

#[cfg(feature = "rayon")]
pub mod delta;

#[cfg(feature = "rayon")]
pub use delta::DeltaStepping;

/// A [`Process`] allowing to apply some transformations to a [`super::Node`]
pub trait Process {
    /// The underlying [`super::Node`] type of this [`Process`]
    type Node;

    /// Build a [`Process`] from a [`super::Node`]
    fn from_node(node: Self::Node) -> Self;
}

/// A [`Process`] allowing to check a graph contains any [`super::Node`]
/// verifying a given predicate
pub trait Contains<I, P>: Process
where
    P: Fn(I) -> bool,
{
    /// Check if a graph contains any [`super::Node`] verifying the given
    /// predicate _pred_
    fn contains(&self, pred: P) -> bool;
}

/// A [`Process`] allowing to find any [`super::Node`] verifying a given
/// predicate
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

/// A [`Process`] allowing to find the first [`super::Node`] verifying a given
/// predicate
pub trait FindFirst<I, P>: Process
where
    P: Fn(I) -> bool,
{
    /// Search for the sequentially **first** item in the parallel iterator that
    /// matches the given predicate and return it
    ///
    /// If you just want the first match that discovered anywhere in the graph,
    /// [`.find_any()`](FindAny::find_any) is a better choice.
    ///
    /// For a weighted graph, the _first_ element is the one with the lowest
    /// distance from the start node.
    fn find_first(&self, pred: P) -> Option<Self::Node>;
}
