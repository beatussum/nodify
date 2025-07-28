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
For the time being, only the followings are supported:
- `Contains` allowing to check whether any node verifying a given predicate;
- `FindAny` allowing to find any node verifying a given predicate;
- `FindFirst` allowing to find the _first node_ (i.e. the one with the shortest path) verifying a given predicate.

`Contains` and `FindAny` is implemented using [DFS](https://en.wikipedia.org/wiki/Depth-first_search) with a sequential variant and a parallel one. A [delta stepping algorithm](https://en.wikipedia.org/wiki/Parallel_single-source_shortest_path_algorithm#Delta_stepping_algorithm) implements `Contains`, `FindFirst` and `FindAny`.

With this _crate_, you just need to implement the `Node` trait with the `outgoing()` method to be able to apply processes.
All processes and process implementations are stored under `nodify::process::*`.

## Example

```rust
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
        .to_process::<DFS<_>>()
        .contains(|FiboNode { current, .. }| current == 610);

    println!("{first:?} => {result}");
}
```

You can consult this example at `examples/fibonacci.rs`.

## Building

First, you need to have a Rust toolchain installed.
You can follow the instructions at [this page](https://www.rust-lang.org/learn/get-started).
If you are a **GNU/Linux** user, it should be included in the official repositories of your favorite distribution.
This project is based on the [Cargo](https://doc.rust-lang.org/cargo/) package manager.

### Features

- `rayon`: to support algorithms using [Rayon](https://github.com/rayon-rs/rayon).

### Using as a dependency

```bash
cargo add nodify
```

### Building from source

#### Dependencies

- The Rust toolchain (**build**)
- [Git](https://git-scm.com/) (**build**)

#### Building process

1. Clone this repository.

   ```bash
   git clone "https://github.com/beatussum/nodify.git"
   ```

1. Build the crate.

   ```bash
   cargo build -r # (-r for release build), or
   cargo build -r -F <features> # if you want to set non default features
   ```

## Licenses

As explained above, the code of this software is licensed under GPL-3 or any later version.
Details of the rights applying to the various third-party files are described in the `copyright` file in [the Debian `debian/copyright` file format](https://www.debian.org/doc/packaging-manuals/copyright-format/1.0/).
