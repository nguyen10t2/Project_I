#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

use std::vec;
use macroquad::prelude::*;

use crate::constants::TILE_SIZE;
use crate::node::Node;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Tile {
    Path,
    Wall,
    Start,
    Goal,
}

pub struct Maze {
    pub grid: Vec<Vec<Tile>>,
    pub start: Node,
    pub goal: Node,
    pub width: usize,
    pub height: usize,
}

impl Maze {
    pub fn new(w_size: usize, h_size: usize) -> Self {
        let width = w_size;
        let height = h_size;
        let grid = vec![vec![Tile::Wall; width]; height];

        let start = Node { x: 1, y: 1 };
        let goal = Node {
            x: width - 2,
            y: height - 2,
        };

        Maze {
            grid,
            start,
            goal,
            width,
            height,
        }
    }

    pub fn in_bounds(x: isize, y: isize, w: usize, h: usize) -> bool {
        x > 0 && x < (w as isize - 1) && y > 0 && y < (h as isize - 1)
    }

    pub fn draw(&self) {
        for y in 0..self.height {
            for x in 0..self.width {
                let color = match self.grid[y][x] {
                    Tile::Wall => BLACK,
                    Tile::Path => WHITE,
                    Tile::Start => GREEN,
                    Tile::Goal => RED,
                };

                draw_rectangle(
                    x as f32 * TILE_SIZE,
                    y as f32 * TILE_SIZE,
                    TILE_SIZE,
                    TILE_SIZE,
                    color,
                );
            }
        }
    }
}
