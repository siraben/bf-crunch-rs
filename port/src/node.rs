//! Lightweight representation of solver nodes, capturing pointer positions and
//! incremental costs.

use std::fmt;

#[derive(Clone, Copy, Debug)]
/// Tape mutation with an associated traversal cost and optional rolling flag.
pub struct Node {
    /// Tape pointer targeted by this mutation.
    pub pointer: i32,
    /// Accumulated cost incurred to reach this pointer.
    pub cost: i32,
    /// Indicates that the node was produced by a rolling optimization step.
    #[allow(dead_code)]
    pub rolling: bool,
}

impl Node {
    /// Creates a non-rolling node at the given pointer with the supplied cost.
    pub fn new(pointer: i32, cost: i32) -> Self {
        Self {
            pointer,
            cost,
            rolling: false,
        }
    }

    /// Creates a node flagged as originating from a rolling sequence.
    pub fn new_with_rolling(pointer: i32, cost: i32) -> Self {
        Self {
            pointer,
            cost,
            rolling: true,
        }
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.pointer == other.pointer
    }
}

impl Eq for Node {}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({} {})", self.pointer, self.cost)
    }
}
