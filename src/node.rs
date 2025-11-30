#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Node {
    pub x: usize,
    pub y: usize,
}

impl Node {
    pub fn new(x: usize, y: usize) -> Self {
        Node { x, y}
    }
}