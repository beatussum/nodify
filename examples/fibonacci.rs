//! Nodify the Fibonnacci sequence

use nodify::prelude::*;
use std::iter::once;

/// A node representing the current state of the sequence
///
/// This trait need to be [`Eq`] and [`Hash`] due to the [`DFS`] process implementation used. It
/// needs also to be [`Copy`] in order to be used without reference.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct FiboNode {
    /// The previous term value
    pub previous: u64,

    /// The current term value
    pub current: u64,
}

impl FiboNode {
    /// Get the first [node](FiboNode) of the sequence
    ///
    /// It corresponds to the two first values of the sequence.
    pub fn first() -> Self {
        Self {
            previous: 0,
            current: 1,
        }
    }
}

impl Node for FiboNode {
    fn outgoing(self) -> impl Iterator<Item = Self> {
        let next = Self {
            previous: self.current,
            current: self.previous + self.current,
        };

        once(next)
    }
}

fn main() {
    let first = FiboNode::first();

    let result = first
        .process::<DFS<_>>()
        .contains_any(|FiboNode { current, .. }| current == 610);

    println!("{first:?} => {result}");
}
