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

/// A trait allowing to convert a [`Node`] to a given type
///
/// This trait aims to support [`Process`] predicate with an input type
/// different from the node type.
pub trait AsValue<I> {
    /// Convert the [`Node`] to the given type
    fn as_value(self) -> I;
}

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

/// A trait implemention allowing [`AsValue`] to be
/// [reflexive](https://en.wikipedia.org/wiki/Reflexive_relation).
impl<N: Node> AsValue<Self> for N {
    fn as_value(self) -> Self {
        self
    }
}
