use std::fmt;

#[derive(Clone, Copy, Debug)]
pub struct Node {
    pub pointer: i32,
    pub cost: i32,
    #[allow(dead_code)]
    pub rolling: bool,
}

impl Node {
    pub fn new(pointer: i32, cost: i32) -> Self {
        Self {
            pointer,
            cost,
            rolling: false,
        }
    }

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
