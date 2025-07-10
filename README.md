# nodify

## Table of contents

- [Description](#description)
- [Example](#example)
- [Building](#building)
  - [Features](#features)
  - [Using as a dependency](#using-as-a-dependency)
  - [Building from source](#building-from-source)
    - [Dependencies](#dependencies)
    - [Building process](#building-process)
- [Licenses](#licenses)

## Description

This small _crate_ aims to provide a easy way to apply computations on problem using graph to be resolved.
For the time being, only `ContainsAny` process is supported allowing to find any node verifying a given predicate. This process is implemented using [DFS](https://en.wikipedia.org/wiki/Depth-first_search) with a sequential variant and a parallel one.

With this _crate_, you just need to implement the `Node` trait with the `outgoing()` method to be able to apply process.
All processes and process implementations are stored under `nodify::process::*`

## Example

```rust
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
        .process::<ParallelDFS<_>>()
        .contains_any(|FrogNode { position, .. }| position == has_stone.len() - 1);

    let stop = start.elapsed();

    println!("{root:?}");
    println!("=> {is_solvable} ({stop:?})");
}
```

You can consult this example at [`examples/frog_jump_with_copy_node.rs`](examples/frog_jump_with_copy_node.rs).

## Building

First, you need to have a Rust toolchain installed.
You can follow the instructions at [this page](https://www.rust-lang.org/learn/get-started).
If you are a **GNU/Linux** user, it should be included in the official repositories of your favorite distribution.
This project is based on the [Cargo](https://doc.rust-lang.org/cargo/) package manager.

### Features

- `rayon`: to support algorithms using [Rayon](https://github.com/rayon-rs/rayon).

### Using as a dependency

```console
cargo add nodify
```

### Building from source

#### Dependencies

- The Rust toolchain (**build**)
- [Git](https://git-scm.com/) (**build**)

#### Building process

1. Clone this repository.

```console
git clone "https://github.com/beatussum/nodify.git"
```

1. Build the crate.

```console
cargo build -r # (-r for release build), or
cargo build -r -F <features> # if you want to set non default features
```

## Licenses

As explained above, the code of this software is licensed under GPL-3 or any later version.
Details of the rights applying to the various third-party files are described in the [copyright](copyright) file in [the Debian `debian/copyright` file format](https://www.debian.org/doc/packaging-manuals/copyright-format/1.0/).
