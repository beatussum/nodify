//! # Graphex
//!
//! _Graphex_ means **Graph** **ex**plore.
//!
//! ## Why should I use Graphex?
//!
//! This crate aims to provide an easy way to implement graph algorithms like
//! graph routing and path finding.

pub mod process;
use process::Process;

/// A trait representing a [graph `Node`](Node).
///
/// This trait is based on the [`.children()`](Node::children), which allows to
/// get the children of the [`Node`]. Using this method, it is possible to
/// implement [`Process`es](Process) which rely on this method to travel the
/// graph.
pub trait Node {
    /// The children type
    ///
    /// The idea behind this type is to provide a way for iterating over the
    /// node children.
    type Children<'a>: Iterator
    where
        <Self::Children<'a> as Iterator>::Item: Node,
        Self: 'a;

    /// Get the children of the current Node
    fn children<'a>(&'a self) -> Self::Children<'a>
    where
        <Self::Children<'a> as Iterator>::Item: Node;

    /// Get the associated [`Process`] according to the given `P`
    fn process<'a, P>(&'a self) -> P
    where
        P: Process + From<&'a Self>,
    {
        self.into()
    }
}
