//! Many different processes
//!
//! This module contains several different [`Process`es](Process)

pub mod parallel_dfs;

use crate::Node;

/// A [`Process`] allowing to apply some transformations to a [`crate::Node`]
pub trait Process {
    /// The underlying [`Node`] type of this [`Process`]
    type Node: Node;

    /// Build a [`Process`] from a [`Node`]
    fn from_node(node: Self::Node) -> Self;
}

/// A [`Process`] allowing to check if a graph contains any [`crate::Node`]
/// verifying a given predicate
pub trait ContainsAny: Process {
    /// Check if a graph contains any [`crate::Node`] verifying the given
    /// predicate `pred`
    fn contains_any<P>(&self, pred: P) -> bool
    where
        P: Fn(&Self) -> bool;
}
