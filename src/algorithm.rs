#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

use macroquad::prelude::*;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};
use std::path;

use crate::constants::{COLOR_PATH, TILE_SIZE};
use crate::heuristic::*;
use crate::maze::{Maze, Tile};
use crate::node::Node;

type HeuristicFn = fn(Node, Node) -> f32;

#[derive(PartialEq, Clone, Copy)]
pub struct State {
    pub cost: f32,
    pub pos: Node,
}

impl Eq for State {}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .cost
            .partial_cmp(&self.cost)
            .unwrap_or(Ordering::Equal)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub struct AStarVisualizer {
    pub open: BinaryHeap<State>,
    pub came_from: HashMap<Node, Node>,
    pub g_score: HashMap<Node, f32>,
    pub path: Option<Vec<Node>>,
    pub found: bool,
    pub start: Node,
    pub goal: Node,
}

impl AStarVisualizer {
    pub fn new(maze: &Maze) -> Self {
        let start = maze.start;
        let goal = maze.goal;

        let mut open = BinaryHeap::new();
        let mut g_score = HashMap::new();

        g_score.insert(start, 0.0);
        open.push(State {
            cost: 0.0,
            pos: start,
        });

        Self {
            open,
            came_from: HashMap::new(),
            g_score,
            path: None,
            found: false,
            start,
            goal,
        }
    }

    pub fn step(&mut self, maze: &Maze, heuristic: HeuristicFn) {
        if self.found || self.open.is_empty() {
            return;
        }

        if let Some(State { cost: _, pos }) = self.open.pop() {
            if pos == self.goal {
                self.found = true;
                self.path = Some(Self::reconstruct_path(&self.came_from, pos));
                return;
            }

            let current_g = *self.g_score.get(&pos).unwrap_or(&f32::INFINITY);
            let dirs = [(0, 1), (1, 0), (0, -1), (-1, 0)];

            for (dx, dy) in dirs {
                let nx = pos.x as isize + dx;
                let ny = pos.y as isize + dy;

                if !Maze::in_bounds(nx, ny, maze.width, maze.height) {
                    continue;
                }

                let neighbor = Node::new(nx as usize, ny as usize);

                if maze.grid[neighbor.y][neighbor.x] == Tile::Wall {
                    continue;
                }

                let tentative_g = current_g + 1.0;
                let neighbor_g = *self.g_score.get(&neighbor).unwrap_or(&f32::INFINITY);

                if tentative_g < neighbor_g {
                    self.came_from.insert(neighbor, pos);
                    self.g_score.insert(neighbor, tentative_g);

                    let f_score = tentative_g + heuristic(neighbor, self.goal);
                    self.open.push(State {
                        cost: f_score,
                        pos: neighbor,
                    });
                }
            }
        }
    }

    fn reconstruct_path(came_from: &HashMap<Node, Node>, mut current: Node) -> Vec<Node> {
        let mut total_path = vec![current];
        while let Some(&prev) = came_from.get(&current) {
            current = prev;
            total_path.push(current);
        }
        total_path.reverse();
        total_path
    }

    pub fn draw(&self, maze: &Maze, cell_size: f32) {
        for node in self.came_from.keys() {
            if *node != maze.start && *node != maze.goal {
                draw_rectangle(
                    node.x as f32 * cell_size,
                    node.y as f32 * cell_size,
                    cell_size,
                    cell_size,
                    COLOR_PATH,
                );
            }
        }

        if let Some(path) = &self.path {
            for node in path {
                if *node != maze.start && *node != maze.goal {
                    draw_rectangle(
                        node.x as f32 * cell_size,
                        node.y as f32 * cell_size,
                        cell_size,
                        cell_size,
                        GREEN,
                    );
                }
            }
        }
    }

    pub fn find_path(
        maze: &Maze,
        start: Node,
        goal: Node,
        heuristic: HeuristicFn,
        obstacles: &[crate::obstacle::DynamicObstacle],
    ) -> Option<Vec<Node>> {
        let mut open = BinaryHeap::new();
        let mut g_score = HashMap::new();
        let mut came_from = HashMap::new();

        g_score.insert(start, 0.0);
        open.push(State {
            cost: 0.0,
            pos: start,
        });

        while let Some(State { cost: _, pos }) = open.pop() {
            if pos == goal {
                return Some(Self::reconstruct_path(&came_from, pos));
            }

            let current_g = *g_score.get(&pos).unwrap_or(&f32::INFINITY);
            let dirs = [(0, 1), (1, 0), (0, -1), (-1, 0)];

            for (dx, dy) in dirs {
                let nx = pos.x as isize + dx;
                let ny = pos.y as isize + dy;

                if !Maze::in_bounds(nx, ny, maze.width, maze.height) {
                    continue;
                }

                let neighbor = Node::new(nx as usize, ny as usize);

                if maze.grid[neighbor.y][neighbor.x] == Tile::Wall {
                    continue;
                }

                // Obstacle check
                if obstacles.iter().any(|obs| obs.position == neighbor) && neighbor != goal {
                    continue;
                }

                let tentative_g = current_g + 1.0;
                let neighbor_g = *g_score.get(&neighbor).unwrap_or(&f32::INFINITY);

                if tentative_g < neighbor_g {
                    came_from.insert(neighbor, pos);
                    g_score.insert(neighbor, tentative_g);

                    let f_score = tentative_g + heuristic(neighbor, goal);
                    open.push(State {
                        cost: f_score,
                        pos: neighbor,
                    });
                }
            }
        }
        None
    }
}
