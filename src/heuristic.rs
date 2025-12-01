#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

use crate::{constants::{MAZE_HEIGHT, MAZE_WIDTH}, helper::direction, node::Node};

use std::f32::consts::PI;

pub fn manhattan(a: Node, b: Node) -> f32 {
    (a.x.abs_diff(b.x) + a.y.abs_diff(b.y)) as f32
}

pub fn euclidean(a: Node, b: Node) -> f32 {
    let dx = a.x as isize - b.x as isize;
    let dy = a.y as isize - b.y as isize;
    ((dx * dx + dy * dy) as f32).sqrt()
}

pub fn uniform_cost(_a: Node, _b: Node) -> f32 {
    0.0
}

pub fn euclidean_squared(a: Node, b: Node) -> f32 {
    let dx = a.x as isize - b.x as isize;
    let dy = a.y as isize - b.y as isize;   
    (dx * dx + dy * dy) as f32
}

pub fn weighted_manhattan(a: Node, b: Node) -> f32 {
    let h = (a.x.abs_diff(b.x) + a.y.abs_diff(b.y)) as f32;
    h * 2.5
}

pub fn manhattan_tiebreaker(a: Node, b: Node) -> f32 {
    let h = (a.x.abs_diff(b.x) + a.y.abs_diff(b.y)) as f32;
    let dir = direction(a, b, MAZE_WIDTH, MAZE_HEIGHT);
    
    let ideal_sector = (1.0_f32).atan2(2.0);

    let tiebreaker = (dir - ideal_sector).abs() / (2.0 * PI);

    h + h * tiebreaker * 0.5
}