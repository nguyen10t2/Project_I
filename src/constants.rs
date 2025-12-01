#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

use std::collections::HashMap;

use macroquad::prelude::*;

use crate::node::Node;
use crate::heuristic::*;

type HeuristicFn = fn(Node, Node) -> f32;

pub const HEURISTIC: &[(KeyCode, HeuristicFn, &str)] = &[
    (KeyCode::Key1, mahattan, "Manhattan"),
    (KeyCode::Key2, euclidean, "Euclidean"),
    (KeyCode::Key3, diagonal, "Diagonal"),
    (KeyCode::Key4, uniform_cost, "Uniform Cost Search"),
    (KeyCode::Key5, chebyshev, "Chebyshev"),
    (KeyCode::Key6, euclidean_squared, "Euclidean Squared"),
    (KeyCode::Key7, weighted_manhattan, "Weighted Manhattan"),
    (KeyCode::Key8, manhattan_tiebreaker, "Manhattan with Tiebreaker"),
];

pub const MAZE_HEIGHT: usize = 101;
pub const MAZE_WIDTH: usize = 2 * MAZE_HEIGHT - 1;

pub const UI_HEIGHT: i32 = 120;

const PIXEL_PER_TILE: i32 = 40 - ((MAZE_HEIGHT - 11) as f32 / 2.0 * 38.0 / 46.0) as i32;

pub const WINDOW_WIDTH: i32 = MAZE_WIDTH as i32 * PIXEL_PER_TILE;
pub const WINDOW_HEIGHT: i32 = MAZE_HEIGHT as i32 * PIXEL_PER_TILE + UI_HEIGHT;


pub const TILE_SIZE: f32 = PIXEL_PER_TILE as f32;

pub const STEP_DELAY_SEC: f64 = 0.0;
pub const STEPS_PER_FRAME: usize = 60;

pub const DENSITY: f32 = 0.5;

pub const COLOR_PATH: Color  = Color::new(0.1, 0.8, 1.0, 0.5);
pub const CYAN: Color       = Color::new(0.0, 1.0, 1.0, 1.0);
