use std::collections::VecDeque;
use std::fmt;

use crate::node::Node;

#[derive(Clone, Debug, Default)]
pub struct Path {
    nodes: VecDeque<Node>,
}

impl Path {
    pub fn new() -> Self {
        Self {
            nodes: VecDeque::new(),
        }
    }

    pub fn cost(&self) -> i32 {
        self.nodes.iter().map(|node| node.cost).sum()
    }

    pub fn add_first(&mut self, node: Node) {
        self.nodes.push_front(node);
    }

    pub fn add_last(&mut self, node: Node) {
        self.nodes.push_back(node);
    }

    pub fn extend_back(&mut self, other: &Path) {
        self.nodes.extend(other.nodes.iter().copied());
    }

    pub fn contains(&self, needle: &Node) -> bool {
        self.nodes.iter().any(|node| node == needle)
    }

    pub fn iter(&self) -> impl Iterator<Item = &Node> {
        self.nodes.iter()
    }

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
