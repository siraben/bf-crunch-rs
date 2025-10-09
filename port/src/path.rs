//! Ordered collection of solver nodes representing a single Brainf**k
//! execution path.

use std::collections::VecDeque;
use std::fmt;

use crate::node::Node;

#[derive(Clone, Debug, Default)]
/// Sequence of [`Node`] instances describing the solver's traversal.
pub struct Path {
    nodes: VecDeque<Node>,
}

impl Path {
    /// Creates an empty path.
    pub fn new() -> Self {
        Self {
            nodes: VecDeque::new(),
        }
    }

    /// Computes the total cost of all nodes in the path.
    pub fn cost(&self) -> i32 {
        self.nodes.iter().map(|node| node.cost).sum()
    }

    /// Inserts a node at the beginning of the path.
    pub fn add_first(&mut self, node: Node) {
        self.nodes.push_front(node);
    }

    /// Appends a node at the end of the path.
    pub fn add_last(&mut self, node: Node) {
        self.nodes.push_back(node);
    }

    /// Extends the path with nodes from `other`, preserving order.
    pub fn extend_back(&mut self, other: &Path) {
        self.nodes.extend(other.nodes.iter().copied());
    }

    /// Tests whether the path already contains the specified node.
    pub fn contains(&self, needle: &Node) -> bool {
        self.nodes.iter().any(|node| node == needle)
    }

    /// Returns an iterator over the nodes in the path.
    pub fn iter(&self) -> impl Iterator<Item = &Node> {
        self.nodes.iter()
    }

    /// Returns the number of nodes stored in the path.
    pub fn len(&self) -> usize {
        self.nodes.len()
    }
}

impl fmt::Display for Path {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut first = true;
        for node in &self.nodes {
            if !first {
                write!(f, ", ")?;
            }
            write!(f, "{}", node)?;
            first = false;
        }
        Ok(())
    }
}
