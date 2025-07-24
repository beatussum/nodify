//! # nodify
//!
//! ## Why should I use nodify?
//!
//! This crate aims to provide an easy way to implement graph algorithms like
//! graph routing and path finding.

#![cfg_attr(docsrs, feature(doc_cfg))]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![warn(missing_docs)]

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

/// A trait representing a [weighted graph node](Weighted).
///
/// # Description
///
/// This trait is based on the
/// [`.weighted_outgoing()`](Weighted::weighted_outgoing), which allows to get
/// the outgoing [nodes](Node) of the current one with the weight of the
/// associated edge. Using this method, it is possible to implement
/// [`Process`es](Process) which rely on this method to travel the graph.
///
/// # Relation with [`Node`]
///
/// In general, a struct implementing [`Weighted`] should implement [`Node`] in
/// order to be used. However, you can notice that it is possible to implement
/// [`.outgoing()`](Node::outgoing) using
/// [`.weighted_oudgoing()`](Weighted::weighted_outgoing).
///
/// You can consult the complete version of this example at `examples/knapsack.rs`.
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
/// impl Node for Knapsack<'_> {
///     fn outgoing(self) -> impl Iterator<Item = Self> {
///         self.weighted_outgoing().map(|(_, outgoing)| outgoing)
///     }
/// }
/// ```
pub trait Weighted {
    /// The type used as weight
    type Weight;

    /// Get the outgoing edges of the current [node](Weighted)
    fn weighted_outgoing(self) -> impl Iterator<Item = (Self::Weight, Self)>;
}
