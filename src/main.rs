#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

mod algorithm;
mod constants;
mod heuristic;
mod maze;
mod node;

use macroquad::prelude::*;
use std::thread::sleep;
use std::time::{Duration, Instant};

use crate::algorithm::AStarVisualizer;
use crate::constants::*;
use crate::maze::{Algorithm, Maze, Tile};
use crate::node::Node;

fn window_conf() -> Conf {
    Conf {
        window_title: "Maze Generator".to_owned(),
        window_width: WINDOW_WIDTH,
        window_height: WINDOW_HEIGHT,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut current_algo = Algorithm::RecursiveBacktracker;

    let mut maze = Maze::new(MAZE_WIDTH, MAZE_HEIGHT, current_algo);

    let mut visualizer = AStarVisualizer::new(&maze);

    let step_delay = STEP_DELAY_SEC;
    let mut time_accumulator = 0f64;

    let mut current_heuristic: fn(Node, Node) -> f32 = heuristic::mahattan;
    let mut heuristic_name = "Manhattan";

    let mut start_time = Instant::now();
    let mut elapsed_duration = Duration::ZERO;
    let mut steps_count = 0;

    loop {
        clear_background(LIGHTGRAY);

        for (key, func, name) in HEURISTIC {
            if is_key_pressed(*key) {
                current_heuristic = *func;
                heuristic_name = *name;
                visualizer = AStarVisualizer::new(&maze);
                time_accumulator = 0.0;

                start_time = Instant::now();
                elapsed_duration = Duration::ZERO;
                steps_count = 0;
                break;
            }
        }

        if is_key_pressed(KeyCode::R) {
            current_algo = Algorithm::RecursiveBacktracker;
            maze = Maze::new(MAZE_WIDTH, MAZE_HEIGHT, current_algo);
            visualizer = AStarVisualizer::new(&maze);

            start_time = Instant::now();
            elapsed_duration = Duration::ZERO;
            steps_count = 0;
        }

        if is_key_pressed(KeyCode::P) {
            current_algo = Algorithm::Prims;
            maze = Maze::new(MAZE_WIDTH, MAZE_HEIGHT, current_algo);
            visualizer = AStarVisualizer::new(&maze);

            start_time = Instant::now();
            elapsed_duration = Duration::ZERO;
            steps_count = 0;
        }

        if is_key_pressed(KeyCode::B) {
            current_algo = Algorithm::Braid;
            maze = Maze::new(MAZE_WIDTH, MAZE_HEIGHT, current_algo);
            visualizer = AStarVisualizer::new(&maze);
            start_time = Instant::now();
            elapsed_duration = Duration::ZERO;
            steps_count = 0;
        }

        if is_key_pressed(KeyCode::Space) {
            maze = Maze::new(MAZE_WIDTH, MAZE_HEIGHT, current_algo);
            visualizer = AStarVisualizer::new(&maze);

            start_time = Instant::now();
            elapsed_duration = Duration::ZERO;
            steps_count = 0;
        }

        if !visualizer.found {
            elapsed_duration = start_time.elapsed();
            if step_delay <= 0.0001 {
                time_accumulator += get_frame_time() as f64;
                for _ in 0..STEPS_PER_FRAME {
                    visualizer.step(&maze, current_heuristic);
                    steps_count += 1;
                    time_accumulator = 0.0;
                    if visualizer.found {
                        elapsed_duration = start_time.elapsed();
                        break;
                    }
                }
            } else {
                time_accumulator += get_frame_time() as f64;
                if time_accumulator >= step_delay {
                    visualizer.step(&maze, current_heuristic);
                    steps_count += 1;
                    time_accumulator -= step_delay;
                }
            }
        }

        maze.draw();

        visualizer.draw(&maze);

        draw_ui(
            heuristic_name,
            elapsed_duration,
            steps_count,
            visualizer.path.as_ref().map_or(0, |p| p.len()),
            visualizer.found,
        );

        next_frame().await;
    }
}

fn draw_ui(
    heuristic_name: &str,
    elapsed_duration: std::time::Duration,
    steps_count: usize,
    distance: usize,
    found: bool,
) {
    use crate::constants::{MAZE_HEIGHT, TILE_SIZE, UI_HEIGHT, WINDOW_WIDTH};

    let ui_y_start = MAZE_HEIGHT as f32 * TILE_SIZE;

    draw_rectangle(
        0.0,
        ui_y_start,
        WINDOW_WIDTH as f32,
        UI_HEIGHT as f32,
        Color::new(0.1, 0.1, 0.1, 1.0),
    );

    let text_x = 10.0;
    let line_spacing = 25.0;

    draw_text(
        format!("Mode: {}", heuristic_name).as_str(),
        text_x,
        ui_y_start + 25.0,
        20.0,
        WHITE,
    );

    draw_text(
        format!(
            "Time: {:.4}s | Steps: {} | Distance: {}",
            elapsed_duration.as_secs_f32(), 
            steps_count,
            distance,
        )
        .as_str(),
        text_x,
        ui_y_start + 25.0 + line_spacing,
        20.0,
        if found { GREEN } else { LIGHTGRAY },
    );

    draw_text(
        "[R] Backtracker | [P] Prim's Algo | [B] Braid Maze",
        text_x,
        ui_y_start + 25.0 + line_spacing * 2.0,
        20.0,
        CYAN,
    );

    draw_text(
        format!(
            "[1-{}] Change Heuris | [Space] New Maze",
            HEURISTIC.len()
        )
        .as_str(),
        text_x,
        ui_y_start + 25.0 + line_spacing * 3.0,
        20.0,
        GOLD,
    );
}
