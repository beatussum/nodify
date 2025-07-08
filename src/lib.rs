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

/// A trait representing a [graph node](Node).
///
/// This trait is based on the [`.outgoing()`](Node::outgoing), which allows to
/// get the outgoing [node](Node) of the current one. Using this method, it is
/// possible to implement [`Process`es](Process) which rely on this method to
/// travel the graph.
pub trait Node {
    /// The outgoing neighbor type
    ///
    /// The idea behind this type is to provide a way for iterating over the
    /// outgoing [node](Node).
    type Outgoing<'this>: Iterator<Item = Self>
    where
        Self: 'this;

    /// Get the outgoing neighbors of the current [node](Node)
    fn outgoing(&self) -> Self::Outgoing<'_>;

    /// Get the associated [`Process`] according to the given `P`
    fn process<'this, P>(&'this self) -> P
    where
        P: Process + From<&'this Self>,
    {
        self.into()
    }
}
