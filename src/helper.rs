#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

use std::f32::consts::PI;

use crate::constants::*;
use crate::node::Node;

pub fn direction(a: Node, goal: Node, width: usize, height: usize) -> f32 {
    let dx = (goal.x as isize - a.x as isize) as f32;
    let dy = (goal.y as isize - a.y as isize) as f32;

    let angle = dy.atan2(dx);

    let angle = (angle + 2.0 * PI) % (2.0 * PI);

    angle
}

pub fn find_set(parent: &mut Vec<usize>, i: usize) -> usize {
    if parent[i] != i {
        parent[i] = find_set(parent, parent[i]);
    }
    parent[i]
}

pub fn union_sets(parent: &mut Vec<usize>, size: &mut Vec<usize>, x: usize, y: usize) {
    let mut xroot = find_set(parent, x);
    let mut yroot = find_set(parent, y);
    if xroot != yroot {
        if size[xroot] < size[yroot] {
            std::mem::swap(&mut xroot, &mut yroot);
        }
        parent[yroot] = xroot;
        size[xroot] += size[yroot];
    }
}