#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

use crate::{constants::{MAZE_HEIGHT, MAZE_WIDTH}, node::Node};

use std::f32::consts::PI;

pub fn mahattan(a: Node, b: Node) -> f32 {
    (a.x.abs_diff(b.x) + a.y.abs_diff(b.y)) as f32
}

pub fn euclidean(a: Node, b: Node) -> f32 {
    let dx = a.x as isize - b.x as isize;
    let dy = a.y as isize - b.y as isize;
    ((dx * dx + dy * dy) as f32).sqrt()
}

pub fn diagonal(a: Node, b: Node) -> f32 {
    let dx = a.x.abs_diff(b.x) as isize;
    let dy = a.y.abs_diff(b.y) as isize;
    let f = (2f32).sqrt() - 1f32;
    if dx < dy {
        f * dx as f32 + dy as f32
    } else {
        f * dy as f32 + dx as f32
    }
}

pub fn uniform_cost(_a: Node, _b: Node) -> f32 {
    0.0
}

pub fn chebyshev(a: Node, b: Node) -> f32 {
    let dx = a.x.abs_diff(b.x);
    let dy = a.y.abs_diff(b.y);
    dx.max(dy) as f32
}

pub fn euclidean_squared(a: Node, b: Node) -> f32 {
    let dx = a.x as isize - b.x as isize;
    let dy = a.y as isize - b.y as isize;   
    (dx * dx + dy * dy) as f32
}

pub fn weighted_manhattan(a: Node, b: Node) -> f32 {
    let h = (a.x.abs_diff(b.x) + a.y.abs_diff(b.y)) as f32;
    h * 5.0
}

pub fn direction(a: Node, goal: Node, width: usize, height: usize) -> f32 {
    let dx = (goal.x as isize - a.x as isize) as f32;
    let dy = (goal.y as isize - a.y as isize) as f32;

    let angle = dy.atan2(dx);

    let angle = (angle + 2.0 * PI) % (2.0 * PI);

    angle
}

pub fn manhattan_tiebreaker(a: Node, b: Node) -> f32 {
    let h = (a.x.abs_diff(b.x) + a.y.abs_diff(b.y)) as f32;
    let dir = direction(a, b, MAZE_WIDTH, MAZE_HEIGHT);
    
    let ideal_sector = (1.0_f32).atan2(2.0);

    let tiebreaker = (dir - ideal_sector).abs() / (2.0 * PI);

    h + h * tiebreaker
}