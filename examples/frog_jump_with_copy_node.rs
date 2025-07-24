//! This example is based on the [403. Frog Jump](https://leetcode.com/problems/frog-jump/) LeetCode problem
//!
//! In this example, [`FrogNode`] is [`Copy`iable](Copy).

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
/// Moreover, the `has_stone` slice is used to compute outgoing nodes according to the stone
/// configuration.
#[derive(Clone, Copy, Debug, Eq)]
pub struct FrogNode<'a> {
    /// The frog's position
    pub position: usize,

    /// The frog's speed
    pub speed: usize,

    /// The stone configuration
    pub has_stone: &'a [bool],
}

/// [`PartialEq`] trait implementation
///
/// Only the frog's position and speed are used.
impl PartialEq for FrogNode<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.position == other.position && self.speed == other.speed
    }
}

/// [`Hash`] trait implementation
///
/// Only the frog's position and speed are used.
impl Hash for FrogNode<'_> {
    fn hash<H>(&self, state: &mut H)
    where
        H: Hasher,
    {
        (self.position, self.speed).hash(state);
    }
}

impl Node for FrogNode<'_> {
    fn outgoing(self) -> impl Iterator<Item = Self> {
        let small_speed = self.speed - 1;
        let big_speed = self.speed + 1;
        let big_position = self.position + big_speed;

        Some((big_position, big_speed))
            .into_iter()
            .chain((small_speed > 0).then_some((self.position + small_speed, small_speed)))
            .chain(Some((self.position + self.speed, self.speed)))
            .filter(|&(p, _)| self.has_stone.get(p).copied().unwrap_or(false))
            .map(move |(position, speed)| Self {
                position,
                speed,
                has_stone: self.has_stone,
            })
    }
}

fn main() {
    let has_stone = (2..1_000_000).map(|_| random_bool(0.8));

    let has_stone = once(true)
        .chain(has_stone)
        .chain(once(true))
        .collect::<Vec<_>>();

    let root = FrogNode {
        position: 0,
        speed: 1,
        has_stone: &has_stone,
    };

    let start = Instant::now();

    let is_solvable = root
        .to_process::<ParallelDFS<_>>()
        .contains(|FrogNode { position, .. }| position == has_stone.len() - 1);

    let stop = start.elapsed();

    println!("{root:?}");
    println!("=> {is_solvable} ({stop:?})");
}
