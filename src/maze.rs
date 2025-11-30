#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

use rand::Rng;
use rand::seq::SliceRandom;

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
        let mut grid = vec![vec![Tile::Wall; width]; height];

        let start = Node { x: 1, y: 1 };
        let goal = Node {
            x: width - 2,
            y: height - 2,
        };

        grid[start.y][start.x] = Tile::Path;

        let mut rng = rand::rng();
        Maze::recursive_backtracker(&mut grid, &mut rng, start.x, start.y, width, height);

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

    pub fn recursive_backtracker(
        grid: &mut Vec<Vec<Tile>>,
        rng: &mut rand::rngs::ThreadRng,
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
}
