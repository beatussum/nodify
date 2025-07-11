//! # nodify
//!
//! ## Why should I use nodify?
//!
//! This crate aims to provide an easy way to implement graph algorithms like
//! graph routing and path finding.

#![cfg_attr(docsrs, feature(doc_cfg))]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]

pub mod nodifyied;
pub mod prelude;
pub mod process;

use process::Process;

/// A trait representing a [graph node](Node).
///
/// This trait is based on the [`.outgoing()`](Node::outgoing), which allows to
/// get the outgoing [node](Node) of the current one. Using this method, it is
/// possible to implement [`Process`es](Process) which rely on this method to
/// travel the graph.
pub trait Node {
    /// Get the associated [`Process`] according to the given `P`
    fn as_process<P>(self) -> P
    where
        P: Process<Node = Self>,
        Self: Sized,
    {
        Process::from_node(self)
    }

    /// Get the outgoing neighbors of the current [node](Node)
    fn outgoing(self) -> impl Iterator<Item = Self>;
}
