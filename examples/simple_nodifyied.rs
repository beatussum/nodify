//! _Nodify_ an integer using an arithmetic progression

use nodify::prelude::*;
use std::iter::once;

fn main() {
    let found = NodifyiedBuilder::new(|i| once(i + 1))
        .build(0)
        .to_process::<DFS<_>>()
        .contains(|i: i32| i == 42);

    println!("{found}");
}
