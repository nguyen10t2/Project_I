use std::collections::HashMap;
use std::vec;

use ::rand::Rng;
use ::rand::prelude::IndexedRandom;
use ::rand::seq::SliceRandom;

use crate::constants::DENSITY;
use crate::maze::{Maze, Tile};
use crate::node::Node;

#[derive(Clone, Copy, PartialEq)]
pub enum Algorithm {
    RecursiveBacktracker,
    Prims,
    Braid,
    Eller,
}

#[derive(Clone, PartialEq)]
pub enum EllerPhase {
    Initialize,
    Horizontal { index: usize },
    Vertical {
        verticals: Vec<usize>,
        current_idx: usize,
        next_row_sets: Vec<usize>,
    },
}

pub enum GeneratorState {
    RecursiveBacktracker {
        stack: Vec<Node>,
    },
    Prims {
        frontier: Vec<Node>,
    },
    Eller {
        row: usize,
        sets: Vec<usize>,
        next_set_id: usize,
        cols: Vec<usize>,
        phase: EllerPhase,
    },
    AddingCycles {
        dead_ends: Vec<Node>,
        current_index: usize,
        target_count: usize,
    },
    Finished,
}

pub struct MazeVisualizer {
    pub state: GeneratorState,
    pub algorithm: Algorithm,
    pub done: bool,
}

impl MazeVisualizer {
    pub fn new(maze: &mut Maze, algo: Algorithm) -> Self {
        for y in 0..maze.height {
            for x in 0..maze.width {
                maze.grid[y][x] = Tile::Wall;
            }
        }

        let start = maze.start;
        let width = maze.width;
        let height = maze.height;

        let state = match algo {
            Algorithm::RecursiveBacktracker => {
                maze.grid[start.y][start.x] = Tile::Path;
                GeneratorState::RecursiveBacktracker { stack: vec![start] }
            }
            Algorithm::Prims | Algorithm::Braid => {
                maze.grid[start.y][start.x] = Tile::Path;
                let mut frontier = Vec::new();
                let directions = [(0, -2), (2, 0), (0, 2), (-2, 0)];
                for (dx, dy) in directions {
                    let nx = start.x as isize + dx;
                    let ny = start.y as isize + dy;
                    if Maze::in_bounds(nx, ny, width, height) {
                        frontier.push(Node {
                            x: nx as usize,
                            y: ny as usize,
                        });
                    }
                }
                GeneratorState::Prims { frontier }
            }
            Algorithm::Eller => {
                let cols: Vec<usize> = (1..width - 1).step_by(2).collect();
                let num_cols = cols.len();
                let sets: Vec<usize> = (0..num_cols).collect();
                GeneratorState::Eller {
                    row: 1,
                    sets,
                    next_set_id: num_cols,
                    cols,
                    phase: EllerPhase::Initialize,
                }
            }
        };

        MazeVisualizer {
            state,
            algorithm: algo,
            done: false,
        }
    }

    pub fn step(&mut self, maze: &mut Maze) {
        let mut rng = ::rand::rng();
        let width = maze.width;
        let height = maze.height;

        match &mut self.state {
            GeneratorState::RecursiveBacktracker { stack } => {
                if let Some(&current) = stack.last() {
                    let directions = &[(0, -2), (2, 0), (0, 2), (-2, 0)];
                    let mut neighbors = Vec::new();

                    for (dx, dy) in directions {
                        let nx = current.x as isize + dx;
                        let ny = current.y as isize + dy;

                        if Maze::in_bounds(nx, ny, width, height) {
                            let nx = nx as usize;
                            let ny = ny as usize;
                            if maze.grid[ny][nx] == Tile::Wall {
                                neighbors.push((nx, ny, dx, dy));
                            }
                        }
                    }

                    if let Some(&(nx, ny, dx, dy)) = neighbors.choose(&mut rng) {
                        let wall_x = (current.x as isize + dx / 2) as usize;
                        let wall_y = (current.y as isize + dy / 2) as usize;

                        maze.grid[wall_y][wall_x] = Tile::Path;
                        maze.grid[ny][nx] = Tile::Path;
                        stack.push(Node { x: nx, y: ny });
                    } else {
                        stack.pop();
                    }
                } else {
                    self.done = true;
                    self.state = GeneratorState::Finished;
                }
            }
            GeneratorState::Prims { frontier } => {
                if !frontier.is_empty() {
                    let rand_index = rng.random_range(0..frontier.len());
                    let current = frontier.swap_remove(rand_index);

                    if maze.grid[current.y][current.x] == Tile::Path {
                        return;
                    }

                    maze.grid[current.y][current.x] = Tile::Path;

                    let directions = [(0, -2), (2, 0), (0, 2), (-2, 0)];
                    let mut neighbors = Vec::new();
                    for (dx, dy) in directions {
                        let nx = current.x as isize + dx;
                        let ny = current.y as isize + dy;

                        if Maze::in_bounds(nx, ny, width, height) {
                            if maze.grid[ny as usize][nx as usize] == Tile::Path {
                                neighbors.push(Node {
                                    x: nx as usize,
                                    y: ny as usize,
                                });
                            }
                        }
                    }

                    if let Some(neighbor) = neighbors.choose(&mut rng) {
                        let wall_x = (current.x + neighbor.x) / 2;
                        let wall_y = (current.y + neighbor.y) / 2;
                        maze.grid[wall_y][wall_x] = Tile::Path;
                    }

                    for (dx, dy) in directions {
                        let nx = current.x as isize + dx;
                        let ny = current.y as isize + dy;

                        if Maze::in_bounds(nx, ny, width, height) {
                            if maze.grid[ny as usize][nx as usize] == Tile::Wall {
                                frontier.push(Node {
                                    x: nx as usize,
                                    y: ny as usize,
                                });
                            }
                        }
                    }
                } else {
                    if self.algorithm == Algorithm::Braid {
                        self.start_adding_cycles(maze);
                    } else {
                        self.done = true;
                        self.state = GeneratorState::Finished;
                    }
                }
            }
            GeneratorState::Eller {
                row,
                sets,
                next_set_id,
                cols,
                phase,
            } => {
                if *row < height - 1 {
                    let r = *row;
                    let num_cols = cols.len();
                    let last_row = r + 2 >= height - 1;

                    match phase {
                        EllerPhase::Initialize => {
                            for i in 0..num_cols {
                                maze.grid[r][cols[i]] = Tile::Path;
                            }
                            *phase = EllerPhase::Horizontal { index: 0 };
                        }
                        EllerPhase::Horizontal { index } => {
                            if *index < num_cols - 1 {
                                let i = *index;
                                let col = cols[i];
                                if sets[i] != sets[i + 1] {
                                    let should_merge = if last_row {
                                        true
                                    } else {
                                        rng.random_bool(DENSITY as f64)
                                    };
                                    if should_merge {
                                        maze.grid[r][col + 1] = Tile::Path;
                                        let old_set = sets[i + 1];
                                        let new_set = sets[i];
                                        for j in 0..num_cols {
                                            if sets[j] == old_set {
                                                sets[j] = new_set;
                                            }
                                        }
                                    }
                                }
                                *index += 1;
                            } else {
                                if last_row {
                                    if self.algorithm == Algorithm::Eller {
                                        self.start_adding_cycles(maze);
                                    } else {
                                        self.done = true;
                                        self.state = GeneratorState::Finished;
                                    }
                                } else {
                                    let mut set_to_cols: HashMap<usize, Vec<usize>> = HashMap::new();
                                    for i in 0..num_cols {
                                        set_to_cols.entry(sets[i]).or_default().push(i);
                                    }

                                    let mut next_row_sets: Vec<usize> = Vec::with_capacity(num_cols);
                                    for _ in 0..num_cols {
                                        next_row_sets.push(*next_set_id);
                                        *next_set_id += 1;
                                    }

                                    let mut verticals = Vec::new();

                                    for (set_id, connected_cols) in set_to_cols.iter() {
                                        let mut shuffled_cols = connected_cols.clone();
                                        shuffled_cols.shuffle(&mut rng);
                                        let mut connected_count = 0;

                                        for &col_idx in shuffled_cols.iter() {
                                            let should_connect = if connected_count == 0 {
                                                true
                                            } else {
                                                rng.random_bool(DENSITY as f64)
                                            };

                                            if should_connect {
                                                verticals.push(col_idx);
                                                next_row_sets[col_idx] = *set_id;
                                                connected_count += 1;
                                            }
                                        }
                                    }
                                    verticals.shuffle(&mut rng);

                                    *phase = EllerPhase::Vertical {
                                        verticals,
                                        current_idx: 0,
                                        next_row_sets,
                                    };
                                }
                            }
                        }
                        EllerPhase::Vertical {
                            verticals,
                            current_idx,
                            next_row_sets,
                        } => {
                            if *current_idx < verticals.len() {
                                let col_idx = verticals[*current_idx];
                                let col = cols[col_idx];
                                maze.grid[r + 1][col] = Tile::Path;
                                maze.grid[r + 2][col] = Tile::Path;
                                *current_idx += 1;
                            } else {
                                *sets = next_row_sets.clone();
                                *row += 2;
                                *phase = EllerPhase::Initialize;
                            }
                        }
                    }
                } else {
                    if self.algorithm == Algorithm::Eller {
                        self.start_adding_cycles(maze);
                    } else {
                        self.done = true;
                        self.state = GeneratorState::Finished;
                    }
                }
            }
            GeneratorState::AddingCycles {
                dead_ends,
                current_index,
                target_count,
            } => {
                if *current_index < *target_count && *current_index < dead_ends.len() {
                    let node = dead_ends[*current_index];
                    *current_index += 1;

                    let jump_dirs = [(0, -2), (0, 2), (-2, 0), (2, 0)];
                    let mut potential_walls = Vec::new();

                    for (dx, dy) in jump_dirs {
                        let nx = node.x as isize + dx;
                        let ny = node.y as isize + dy;

                        if Maze::in_bounds(nx, ny, width, height) {
                            if maze.grid[ny as usize][nx as usize] == Tile::Path {
                                potential_walls.push((dx, dy));
                            }
                        }
                    }

                    if let Some((dx, dy)) = potential_walls.choose(&mut rng) {
                        let wall_x = node.x as isize + dx / 2;
                        let wall_y = node.y as isize + dy / 2;

                        if Maze::in_bounds(wall_x, wall_y, width, height) {
                            maze.grid[wall_y as usize][wall_x as usize] = Tile::Path;
                        }
                    }
                } else {
                    self.done = true;
                    self.state = GeneratorState::Finished;
                }
            }
            GeneratorState::Finished => {
                self.done = true;
            }
        }

        if self.done {
            maze.grid[maze.start.y][maze.start.x] = Tile::Start;
            maze.grid[maze.goal.y][maze.goal.x] = Tile::Goal;
        }
    }

    fn start_adding_cycles(&mut self, maze: &Maze) {
        let mut dead_ends: Vec<Node> = Vec::new();
        let directions = [(0, 1), (0, -1), (1, 0), (-1, 0)];
        let width = maze.width;
        let height = maze.height;

        for y in 1..height - 1 {
            for x in 1..width - 1 {
                if maze.grid[y][x] == Tile::Path {
                    let mut wall_count = 0;
                    for (dx, dy) in &directions {
                        let nx = x as isize + dx;
                        let ny = y as isize + dy;
                        if Maze::in_bounds(nx, ny, width, height) {
                            if maze.grid[ny as usize][nx as usize] == Tile::Wall {
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

        let mut rng = ::rand::rng();
        dead_ends.shuffle(&mut rng);
        let target_count = (dead_ends.len() as f32 * DENSITY) as usize;

        self.state = GeneratorState::AddingCycles {
            dead_ends,
            current_index: 0,
            target_count,
        };
    }
}
