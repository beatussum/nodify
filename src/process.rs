//! Many different processes
//!
//! This modules contains several different [`Process`es](Process)

pub mod parallel_dfs;

/// A [`Process`] allowing to apply some transformations to a [`crate::Node`]
///
/// This trait is just a [marker](std::marker).
pub trait Process {}

/// A [`Process`] allowing to check if a graph contains any [`crate::Node`]
/// verifying a given predicate
pub trait ContainsAny: Process {
    /// Check if a graph contains any [`crate::Node`] verifying the given
    /// predicate `pred`
    fn contains_any<P>(&self, pred: P) -> bool
    where
        P: Fn(&Self) -> bool;
}
