//! This module provides all the necessary things to _nodify_ a given type
//!
//! # Example
//!
//! You can consult the complete version of this example at `examples/nodifyied.rs`.
//!
//! ```
//! # use nodify::prelude::*;
//! # use std::iter::once;
//! #
//! # #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
//! # pub struct FiboNode {
//! #     pub previous: u64,
//! #     pub current: u64,
//! # }
//! #
//! # impl FiboNode {
//! #     pub fn first() -> Self {
//! #         Self {
//! #             previous: 0,
//! #             current: 1,
//! #         }
//! #     }
//! # }
//! #
//! fn main() {
//!     let first = FiboNode::first();
//!
//!     let result = first
//!         .nodifyied_with(&|FiboNode { previous, current }| {
//!             let next = FiboNode {
//!                 previous: current,
//!                 current: previous + current,
//!             };
//!             once(next)
//!         })
//!         .as_process::<DFS<_>>()
//!         .contains(|FiboNode { current, .. }| current == 610);
//!
//!     println!("{first:?} => {result}");
//! }
//! ```

use crate::{AsValue, Node};
use std::hash::{Hash, Hasher};

/// An easy way to _nodify_ an entity given a _closure_
#[derive(Debug, Clone, Copy)]
pub struct Nodifyied<'a, C, F> {
    current: C,
    outgoing_wrapper: &'a F,
}

impl<C: PartialEq, F> PartialEq for Nodifyied<'_, C, F> {
    fn eq(&self, other: &Self) -> bool {
        self.current == other.current
    }
}

impl<C: Eq, F> Eq for Nodifyied<'_, C, F> {}

impl<C: Hash, F> Hash for Nodifyied<'_, C, F> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.current.hash(state);
    }
}

/// An extension trait allowing to easily _nodify_ a type
pub trait NodifyiedWith {
    /// _Nodify_ the given type
    ///
    /// # Arguments
    ///
    /// - `outgoing_wrapper` - The closure allowing to get the outgoing nodes from the current one.
    ///
    /// # Return value
    ///
    /// A [_nodifyied_](Nodifyied) type
    fn nodifyied_with<F>(self, outgoing_wrapper: &F) -> Nodifyied<'_, Self, F>
    where
        Self: Sized,
    {
        Nodifyied {
            current: self,
            outgoing_wrapper,
        }
    }
}

impl<T> NodifyiedWith for T {}

/// A trait implementation allowing casting to the underlying type
///
/// # Example
///
/// You can consult the complete version of this example at `examples/nodifyied.rs`.
///
/// ```
/// # use nodify::prelude::*;
/// # use std::iter::once;
/// #
/// # #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
/// # pub struct FiboNode {
/// #     pub previous: u64,
/// #     pub current: u64,
/// # }
/// #
/// # impl FiboNode {
/// #     pub fn first() -> Self {
/// #         Self {
/// #             previous: 0,
/// #             current: 1,
/// #         }
/// #     }
/// # }
/// #
/// fn main() {
///     let first = FiboNode::first();
///
///     let result = first
///         .nodifyied_with(&|FiboNode { previous, current }| {
///             let next = FiboNode {
///                 previous: current,
///                 current: previous + current,
///             };
///             once(next)
///         })
///         .as_process::<DFS<_>>()
///         .contains(|FiboNode { current, .. }| current == 610);
///
///     println!("{first:?} => {result}");
/// }
/// ```
///
/// In the above example, you can see that
/// [`.contains()`](crate::process::Contains::contains) takes a `FiboNode` and
/// not a [`Nodifyied`].
impl<C, F> AsValue<C> for Nodifyied<'_, C, F> {
    fn as_value(self) -> C {
        self.current
    }
}

impl<C, F, R> Node for Nodifyied<'_, C, F>
where
    C: Copy,
    F: Copy + Fn(C) -> R,
    R: Iterator<Item = C>,
{
    fn outgoing(self) -> impl Iterator<Item = Self> {
        (*self.outgoing_wrapper)(self.current).map(|current| Nodifyied {
            current,
            outgoing_wrapper: self.outgoing_wrapper,
        })
    }
}
