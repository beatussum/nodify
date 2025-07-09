//! This example is based on the [403. Frog Jump](https://leetcode.com/problems/frog-jump/) LeetCode problem
//!
//! In this example, [`FrogNode`] is not [`Copy`iable](Copy).

use nodify::prelude::*;
use rand::random_bool;

use std::{
    hash::{Hash, Hasher},
    iter::once,
    time::Instant,
};

/// A node representing the frog state
///
/// This node is characterized by
/// - the frog's position, and
/// - the frog's speed.
///
/// Moreover, the outgoing nodes are cached in order to allow an example where the node cannot be
/// [`Copy`ied](Copy).
#[derive(Clone, Debug, Eq)]
pub struct FrogNode {
    position: usize,
    speed: usize,
    outgoing: Vec<Self>,
}

impl FrogNode {
    /// Create a new [`FrogNode`] from the current frog's position and speed
    ///
    /// `has_stone` is used to build the outgoing nodes, used later in the [`Node`] trait
    /// implementation.
    pub fn new(position: usize, speed: usize, has_stone: &[bool]) -> Self {
        let small_speed = speed - 1;
        let big_speed = speed + 1;
        let big_position = position + big_speed;

        let outgoing = Some((big_position, big_speed))
            .into_iter()
            .chain((small_speed > 0).then_some((position + small_speed, small_speed)))
            .chain(Some((position + speed, speed)))
            .filter(|&(p, _)| has_stone.get(p).copied().unwrap_or(false))
            .map(|(position, speed)| Self::new(position, speed, has_stone))
            .collect();

        Self {
            position,
            speed,
            outgoing,
        }
    }
}

/// [`PartialEq`] trait implementation
///
/// Only the frog's position and speed are used.
impl PartialEq for FrogNode {
    fn eq(&self, other: &Self) -> bool {
        self.position == other.position && self.speed == other.speed
    }
}

/// [`Hash`] trait implementation
///
/// Only the frog's position and speed are used.
impl Hash for FrogNode {
    fn hash<H>(&self, state: &mut H)
    where
        H: Hasher,
    {
        (self.position, self.speed).hash(state);
    }
}

/// [`Node`] trait implementation
///
/// The previously built outgoing nodes are used.
///
/// Please note that the trait is implemented for `& FrogNode` and not `FrogNode`.
impl Node for &FrogNode {
    fn outgoing(self) -> impl Iterator<Item = Self> {
        self.outgoing.iter()
    }
}

fn main() {
    let has_stone = (2..10).map(|_| random_bool(0.8));

    let has_stone = once(true)
        .chain(has_stone)
        .chain(once(true))
        .collect::<Vec<_>>();

    let root = FrogNode::new(0, 1, &has_stone);

    let start = Instant::now();

    let is_solvable = root
        .process::<ParallelDFS<_>>()
        .contains_any(|&FrogNode { position, .. }| position == has_stone.len() - 1);

    let stop = start.elapsed();

    println!("{root:?}");
    println!("=> {is_solvable} ({stop:?})");
}
