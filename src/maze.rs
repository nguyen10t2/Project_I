#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

use std::vec;

use ::rand::Rng;
use ::rand::prelude::IndexedRandom;
use ::rand::seq::SliceRandom;
use macroquad::prelude::*;

use crate::constants::{DENSITY, TILE_SIZE};
use crate::helper::{find_set, union_sets};
use crate::node::Node;

#[derive(Clone, Copy)]
pub enum Algorithm {
    RecursiveBacktracker,
    Prims,
    Braid,
    Eller,
}

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
    pub fn new(w_size: usize, h_size: usize, alo: Algorithm) -> Self {
        let width = w_size;
        let height = h_size;
        let mut grid = vec![vec![Tile::Wall; width]; height];

        let start = Node { x: 1, y: 1 };
        let goal = Node {
            x: width - 2,
            y: height - 2,
        };

        let mut rng = ::rand::rng();

        match alo {
            Algorithm::Prims => {
                Maze::generate_prims(&mut grid, &mut rng, start.x, start.y, width, height);
            }
            Algorithm::RecursiveBacktracker => {
                grid[start.y][start.x] = Tile::Path;
                Maze::recursive_backtracker(&mut grid, &mut rng, start.x, start.y, width, height);
            }
            Algorithm::Braid => {
                grid[start.y][start.x] = Tile::Path;
                Maze::generate_prims(&mut grid, &mut rng, start.x, start.y, width, height);

                Maze::add_cycles(&mut grid, &mut rng, width, height, DENSITY);
            }
            Algorithm::Eller => {
                Maze::generate_eller(&mut grid, &mut rng, width, height);

                Maze::add_cycles(&mut grid, &mut rng, width, height, DENSITY);
            }
        }

        grid[start.y][start.x] = Tile::Start;
        grid[goal.y][goal.x] = Tile::Goal;

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

    pub fn recursive_backtracker(
        grid: &mut Vec<Vec<Tile>>,
        rng: &mut ::rand::rngs::ThreadRng,
        x: usize,
        y: usize,
        width: usize,
        height: usize,
    ) {
        let directions = &[(0, -2), (2, 0), (0, 2), (-2, 0)];
        let mut shuffled_directions = directions.to_vec();
        shuffled_directions.shuffle(rng);

        for (dx, dy) in shuffled_directions {
            let nx = x as isize + dx;
            let ny = y as isize + dy;

            if !Self::in_bounds(nx, ny, width, height) {
                continue;
            }

            let nx = nx as usize;
            let ny = ny as usize;

            if grid[ny][nx] == Tile::Wall {
                let wall_x = x as isize + dx / 2;
                let wall_y = y as isize + dy / 2;

                if !Self::in_bounds(wall_x, wall_y, width, height) {
                    continue;
                }

                let wall_x = wall_x as usize;
                let wall_y = wall_y as usize;

                grid[wall_y][wall_x] = Tile::Path;
                grid[ny][nx] = Tile::Path;

                Maze::recursive_backtracker(grid, rng, nx, ny, width, height);
            }
        }
    }

    pub fn generate_prims(
        grid: &mut Vec<Vec<Tile>>,
        rng: &mut ::rand::rngs::ThreadRng,
        start_x: usize,
        start_y: usize,
        width: usize,
        height: usize,
    ) {
        let mut frontier: Vec<Node> = Vec::new();

        grid[start_y][start_x] = Tile::Path;

        let directions = [(0, -2), (2, 0), (0, 2), (-2, 0)];

        for (dx, dy) in directions {
            let nx = start_x as isize + dx;
            let ny = start_y as isize + dy;

            if Self::in_bounds(nx, ny, width, height) {
                frontier.push(Node {
                    x: nx as usize,
                    y: ny as usize,
                });
            }
        }

        while !frontier.is_empty() {
            let rand_index = rng.random_range(0..frontier.len());
            let current = frontier.remove(rand_index);

            if grid[current.y][current.x] == Tile::Path {
                continue;
            }

            grid[current.y][current.x] = Tile::Path;

            let mut neighbors: Vec<Node> = Vec::new();
            for (dx, dy) in directions {
                let nx = current.x as isize + dx;
                let ny = current.y as isize + dy;

                if Self::in_bounds(nx, ny, width, height) {
                    if grid[ny as usize][nx as usize] == Tile::Path {
                        neighbors.push(Node {
                            x: nx as usize,
                            y: ny as usize,
                        });
                    }
                }
            }

            if let Some(neighbor) = neighbors.choose(rng) {
                let wall_x = (current.x + neighbor.x) / 2;
                let wall_y = (current.y + neighbor.y) / 2;
                grid[wall_y][wall_x] = Tile::Path;
            }

            for (dx, dy) in directions {
                let nx = current.x as isize + dx;
                let ny = current.y as isize + dy;

                if Self::in_bounds(nx, ny, width, height) {
                    if grid[ny as usize][nx as usize] == Tile::Wall {
                        frontier.push(Node {
                            x: nx as usize,
                            y: ny as usize,
                        });
                    }
                }
            }
        }
    }

    pub fn generate_eller(
        grid: &mut Vec<Vec<Tile>>,
        rng: &mut ::rand::rngs::ThreadRng,
        width: usize,
        height: usize,
    ) {
        use std::collections::HashMap;

        let cols: Vec<usize> = (1..width - 1).step_by(2).collect();
        let num_cols = cols.len();

        if num_cols == 0 {
            return;
        }

        let mut cur_set: Vec<usize> = (0..num_cols).collect();
        let mut next_set_id = num_cols;

        for row in (1..height - 1).step_by(2) {
            let last_row = row + 2 >= height - 1;

            for i in 0..num_cols {
                grid[row][cols[i]] = Tile::Path;
            }

            for i in 0..num_cols - 1 {
                let col = cols[i];

                if cur_set[i] != cur_set[i + 1] {
                    let should_merge = if last_row { 
                        true 
                    } else { 
                        rng.random_bool(DENSITY as f64) 
                    };

                    if should_merge {
                        grid[row][col + 1] = Tile::Path;

                        let old_set = cur_set[i + 1];
                        let new_set = cur_set[i];
                        for j in 0..num_cols {
                            if cur_set[j] == old_set {
                                cur_set[j] = new_set;
                            }
                        }
                    }
                }
            }

            if last_row {
                break;
            }

            let mut set_to_cols: HashMap<usize, Vec<usize>> = HashMap::new();
            for i in 0..num_cols {
                set_to_cols
                    .entry(cur_set[i])
                    .or_insert_with(Vec::new)
                    .push(i);
            }

            let mut next_row: Vec<usize> = Vec::with_capacity(num_cols);
            for _ in 0..num_cols {
                next_row.push(next_set_id);
                next_set_id += 1;
            }

            for (set_id, connected_cols) in set_to_cols.iter() {
                let mut shuffled_cols = connected_cols.clone();
                shuffled_cols.shuffle(rng);

                let mut connected_count = 0;

                for &col_idx in shuffled_cols.iter() {
                    let col = cols[col_idx];

                    let should_connect = if connected_count == 0 {
                        true
                    } else {
                        rng.random_bool(DENSITY as f64)
                    };

                    if should_connect {
                        grid[row + 1][col] = Tile::Path;
                        grid[row + 2][col] = Tile::Path;
                        next_row[col_idx] = *set_id;
                        connected_count += 1;
                    }
                }
            }

            cur_set = next_row;
        }
    }
    
    pub fn add_cycles(
        grid: &mut Vec<Vec<Tile>>,
        rng: &mut ::rand::rngs::ThreadRng,
        width: usize,
        height: usize,
        density: f32,
    ) {
        let mut dead_ends: Vec<Node> = Vec::new();

        let directions = [(0, 1), (0, -1), (1, 0), (-1, 0)];

        for y in 1..height - 1 {
            for x in 1..width - 1 {
                if grid[y][x] == Tile::Path {
                    let mut wall_count = 0;
                    for (dx, dy) in &directions {
                        let nx = x as isize + dx;
                        let ny = y as isize + dy;
                        if Self::in_bounds(nx, ny, width, height) {
                            if grid[ny as usize][nx as usize] == Tile::Wall {
                                wall_count += 1;
                            }
                        }
                    }
                    if wall_count >= 3 {
                        dead_ends.push(Node { x, y });
                    }
                }
            }
        }

        dead_ends.shuffle(rng);

        let remove_count = (dead_ends.len() as f32 * density) as usize;

        for i in 0..remove_count {
            let node = dead_ends[i];

            let mut potential_walls = Vec::new();

            let jump_dirs = [(0, -2), (0, 2), (-2, 0), (2, 0)];

            for (dx, dy) in jump_dirs {
                let nx = node.x as isize + dx;
                let ny = node.y as isize + dy;

                if Self::in_bounds(nx, ny, width, height) {
                    if grid[ny as usize][nx as usize] == Tile::Path {
                        potential_walls.push((dx, dy));
                    }
                }
            }

            if let Some((dx, dy)) = potential_walls.choose(rng) {
                let wall_x = node.x as isize + dx / 2;
                let wall_y = node.y as isize + dy / 2;

                if Self::in_bounds(wall_x, wall_y, width, height) {
                    grid[wall_y as usize][wall_x as usize] = Tile::Path;
                }
            }
        }
    }
}
