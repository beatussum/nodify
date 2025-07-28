//! This module provides all the necessary things to _nodify_ a given type
//!
//! # Description
//!
//! This module contains
//! - [`Nodifyied`] which is the _nodifyied_ [node](Node) implementation, and
//! - with [its builder](NodifyiedBuilder).
//!
//! See the documentation of the entities described just above for more
//! information.

use super::{Node, ToValue};
use std::hash::{Hash, Hasher};

/// The builder for [`Nodifyied`]
///
/// # Example
///
/// You can consult the complete version of this example at
/// `examples/simple_nodifyied.rs`.
///
/// ```rust
#[doc = include_str!("../examples/simple_nodifyied.rs")]
/// ```
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct NodifyiedBuilder<F> {
    outgoing_wrapper: F,
}

impl<F> NodifyiedBuilder<F> {
    /// Create a new [`NodifyiedBuilder`]
    ///
    /// # Arguments
    ///
    /// - `outgoing_wrapper` - The [`.outgoing()`](Node::outgoing) wrapper function used
    ///
    /// # Return
    ///
    /// A new [`NodifyiedBuilder`] initialized with the given closure.
    pub fn new(outgoing_wrapper: F) -> Self {
        Self { outgoing_wrapper }
    }

    /// Build the associated [`Nodifyied`] node
    ///
    /// # Arguments
    ///
    /// - `current` - The value of the first node
    ///
    /// # Return
    ///
    /// The built associated [`Nodifyied`] node
    pub fn build<C>(&self, current: C) -> Nodifyied<'_, C, F> {
        Nodifyied {
            current,
            outgoing_wrapper: &self.outgoing_wrapper,
        }
    }

    /// Change the value of the [`.outgoing()`](Node::outgoing) wrapper function
    ///
    /// # Arguments
    ///
    /// - `outgoing_wrapper` - The new value of the [`.outgoing()`](Node::outgoing) wrapper function
    ///
    /// # Return
    ///
    /// The altered [`NodifyiedBuilder`]
    pub fn with_outgoing(&mut self, outgoing_wrapper: F) -> &mut Self {
        self.outgoing_wrapper = outgoing_wrapper;
        self
    }
}

/// A _nodifyied_ node
#[derive(Clone, Copy, Debug)]
pub struct Nodifyied<'a, C, F> {
    current: C,
    outgoing_wrapper: &'a F,
}

/// [`PartialEq`] implementation for [`Nodifyied`]
///
/// The `current` state of the node is transparently compared with.
impl<C: PartialEq, F> PartialEq for Nodifyied<'_, C, F> {
    fn eq(&self, other: &Self) -> bool {
        self.current.eq(&other.current)
    }
}

/// [`Eq`] implementation for [`Nodifyied`]
///
/// The `current` state of the node is transparently compared with.
impl<C: Eq, F> Eq for Nodifyied<'_, C, F> {}

/// [`Hash`] implementation for [`Nodifyied`]
///
/// The `current` state of the node is transparently hashed.
impl<C: Hash, F> Hash for Nodifyied<'_, C, F> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.current.hash(state);
    }
}

/// [`ToValue`] implementation for [`Nodifyied`]
///
/// This implementation allows casting to the underlying type.
///
/// # Example
///
/// You can consult the complete version of this example at `examples/nodifyied.rs`.
///
/// ```rust
#[doc = include_str!("../examples/nodifyied.rs")]
/// ```
///
/// In the above example, you can see that
/// [`.contains()`](crate::process::Contains::contains) takes a `FiboNode` and
/// not a [`Nodifyied`].
impl<C, F> ToValue<C> for Nodifyied<'_, C, F> {
    fn to_value(self) -> C {
        self.current
    }
}

/// [`Node`] implementation for [`Nodifyied`]
///
/// The function-like `outgoing_wrapper` is used to generate the outgoing nodes
/// using the current state.
impl<C, F, R> Node for Nodifyied<'_, C, F>
where
    F: Fn(C) -> R,
    R: Iterator<Item = C>,
{
    fn outgoing(self) -> impl Iterator<Item = Self> {
        (self.outgoing_wrapper)(self.current).map(move |current| Self {
            current,
            outgoing_wrapper: self.outgoing_wrapper,
        })
    }
}
