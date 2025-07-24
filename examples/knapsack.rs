//! This example is based on [this GeeksforGeeks article](https://www.geeksforgeeks.org/dsa/0-1-knapsack-problem-dp-10/)
//! which explores the [Knapsack problem](https://en.wikipedia.org/wiki/Knapsack_problem).

use nodify::prelude::*;
use std::iter::once;

/// A Knapsack item
#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub struct Item {
    /// The item value
    pub value: u32,

    /// The item weight
    pub weight: u32,
}

/// A state of the Knapsack problem
#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub struct Knapsack<'a> {
    capacity: u32,
    items: &'a [Item],
    value: u32,
    max_value: u32,
}

impl<'a> Knapsack<'a> {
    /// Create a new [`Knapsack`] with its [items](Item) and its capacity
    ///
    /// This function fails if `items` is an empty slice.
    pub fn new(capacity: u32, items: &'a [Item]) -> Option<Self> {
        items.iter().max_by_key(|Item { value, .. }| value).map(
            |&Item {
                 value: max_value, ..
             }| {
                Self {
                    capacity,
                    items,
                    value: 0,
                    max_value,
                }
            },
        )
    }

    /// Check if the current state is a solution
    pub fn is_solution(&self) -> bool {
        self.items.is_empty()
    }
}

impl Weighted for Knapsack<'_> {
    type Weight = u32;

    /// Get the next state of the Knapsack problem
    ///
    /// # Next states
    ///
    /// There is at most two next states. They correspond to
    /// - we do not take the current item,
    /// - we take the current item if the capacity is enough.
    ///
    /// # Positive weights
    ///
    /// In order to avoid negative weight, we compute weights from the maximum
    /// item value. In this way, all weights are positive and, as functions of
    /// the item values, are decreasing.
    fn weighted_outgoing(self) -> impl Iterator<Item = (Self::Weight, Self)> {
        self.items
            .split_first()
            .map(|(&Item { weight, value }, items)| {
                once({
                    let node = Self {
                        capacity: self.capacity,
                        items,
                        value: self.value,
                        max_value: self.max_value,
                    };

                    (self.max_value, node)
                })
                .chain((weight <= self.capacity).then(|| {
                    let node = Knapsack {
                        capacity: self.capacity - weight,
                        items,
                        value: self.value + value,
                        max_value: self.max_value,
                    };

                    (self.max_value - value, node)
                }))
            })
            .into_iter()
            .flatten()
    }
}

impl Node for Knapsack<'_> {
    fn outgoing(self) -> impl Iterator<Item = Self> {
        self.weighted_outgoing().map(|(_, outgoing)| outgoing)
    }
}

fn main() -> Result<(), &'static str> {
    let items = [
        Item {
            value: 1,
            weight: 1,
        },
        Item {
            value: 7,
            weight: 2,
        },
        Item {
            value: 11,
            weight: 3,
        },
    ];

    let capacity = 5;
    let root = Knapsack::new(capacity, &items).ok_or("Root creation failed")?;

    let Knapsack {
        mut capacity,
        value,
        ..
    } = root
        .to_process::<DeltaStepping<_, _>>()
        .with_delta(2)
        .find_first(|node| node.is_solution())
        .ok_or("No solution")?;

    capacity = capacity - capacity;

    println!("weight = {capacity}, value = {value}");

    Ok(())
}
