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
    /// The underlying value of the [node](Node)
    ///
    /// Sometimes, it is necessary to have a different _value type_ from the _node type_. In
    /// particular, [`nodifyied`] needs that.
    ///
    /// # Implementation
    ///
    /// Due to language limitations,
    /// [_associated types defaults_](https://github.com/rust-lang/rust/issues/29661) are not yet
    /// supported; for this reason, this associated type is not defaulted to `Self` although this
    /// is the majority of cases.
    ///
    /// ```
    /// use nodify::prelude::*;
    /// use std::iter::empty;
    ///
    /// pub struct FooNode;
    ///
    /// impl Node for FooNode {
    ///     type Value = Self;
    ///
    ///     fn outgoing(self) -> impl Iterator<Item = Self> {
    ///         empty()
    ///     }
    ///
    ///     fn value(self) -> Self::Value {
    ///         self
    ///     }
    /// }
    /// ```
    type Value;

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

    /// Get the underlying value of the [node](Node)
    fn value(self) -> Self::Value;
}
