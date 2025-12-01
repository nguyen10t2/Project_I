#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

mod algorithm;
mod constants;
mod generator;
mod heuristic;
mod maze;
mod node;
mod helper;

use macroquad::prelude::*;
use std::thread::sleep;
use std::time::{Duration, Instant};

use crate::algorithm::AStarVisualizer;
use crate::constants::*;
use crate::generator::{Algorithm, MazeVisualizer};
use crate::maze::{Maze, Tile};
use crate::node::Node;

fn window_conf() -> Conf {
    Conf {
        window_title: "Maze Generator".to_owned(),
        window_width: WINDOW_WIDTH,
        window_height: WINDOW_HEIGHT,
        ..Default::default()
    }
}

enum AppState {
    Generating,
    Solving,
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut current_algo = Algorithm::RecursiveBacktracker;

    let mut maze = Maze::new(MAZE_WIDTH, MAZE_HEIGHT);
    let mut maze_visualizer = MazeVisualizer::new(&mut maze, current_algo);
    let mut app_state = AppState::Generating;
    
    let mut astar_visualizer: Option<AStarVisualizer> = None;

    let step_delay = STEP_DELAY_SEC;
    let mut time_accumulator = 0f64;

    let mut current_heuristic: fn(Node, Node) -> f32 = heuristic::manhattan;
    let mut heuristic_name = "Manhattan";

    let mut start_time = Instant::now();
    let mut elapsed_duration = Duration::ZERO;
    let mut steps_count = 0;

    loop {
        clear_background(LIGHTGRAY);

        let mut reset = false;

        for (key, func, name) in HEURISTIC {
            if is_key_pressed(*key) {
                current_heuristic = *func;
                heuristic_name = *name;
                if let AppState::Solving = app_state {
                    astar_visualizer = Some(AStarVisualizer::new(&maze));
                    time_accumulator = 0.0;
                    start_time = Instant::now();
                    elapsed_duration = Duration::ZERO;
                    steps_count = 0;
                }
                break;
            }
        }

        if is_key_pressed(KeyCode::R) {
            current_algo = Algorithm::RecursiveBacktracker;
            reset = true;
        }

        if is_key_pressed(KeyCode::P) {
            current_algo = Algorithm::Prims;
            reset = true;
        }

        if is_key_pressed(KeyCode::B) {
            current_algo = Algorithm::Braid;
            reset = true;
        }

        if is_key_pressed(KeyCode::Space) {
            reset = true;
        }

        if is_key_pressed(KeyCode::E) {
            current_algo = Algorithm::Eller;
            reset = true;
        }

        if reset {
            maze = Maze::new(MAZE_WIDTH, MAZE_HEIGHT);
            maze_visualizer = MazeVisualizer::new(&mut maze, current_algo);
            app_state = AppState::Generating;
            astar_visualizer = None;
            start_time = Instant::now();
            elapsed_duration = Duration::ZERO;
            steps_count = 0;
            time_accumulator = 0.0;
        }

        if is_key_pressed(KeyCode::A) {
            if let AppState::Generating = app_state {
                while !maze_visualizer.done {
                    maze_visualizer.step(&mut maze);
                }
            }
        }
        
        match app_state {
            AppState::Generating => {
                if !maze_visualizer.done {
                    elapsed_duration = start_time.elapsed();
                    if step_delay <= 0.0001 {
                        for _ in 0..MAZE_GEN_STEPS_PER_FRAME {
                            maze_visualizer.step(&mut maze);
                            steps_count += 1;
                            if maze_visualizer.done {
                                break;
                            }
                        }
                        time_accumulator = 0.0;
                    } else {
                        time_accumulator += get_frame_time() as f64;
                        if time_accumulator >= step_delay {
                            maze_visualizer.step(&mut maze);
                            steps_count += 1;
                            time_accumulator -= step_delay;
                        }
                    }
                }
                
                if maze_visualizer.done {
                    app_state = AppState::Solving;
                    astar_visualizer = Some(AStarVisualizer::new(&maze));
                    start_time = Instant::now();
                    elapsed_duration = Duration::ZERO;
                    steps_count = 0;
                }
            }
            AppState::Solving => {
                if let Some(visualizer) = &mut astar_visualizer {
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
                }
            }
        }

        maze.draw();

        if let Some(visualizer) = &astar_visualizer {
            visualizer.draw(&maze);
        }

        let found = astar_visualizer.as_ref().map_or(false, |v| v.found);
        let distance = astar_visualizer.as_ref().map_or(0, |v| v.path.as_ref().map_or(0, |p| p.len()));
        
        let status_text = match app_state {
            AppState::Generating => "Generating Maze...",
            AppState::Solving => if found { "Solved!" } else { "Solving..." },
        };

        draw_ui(
            heuristic_name,
            elapsed_duration,
            steps_count,
            distance,
            found,
            status_text,
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
    status: &str,
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
        format!("Mode: {} | Status: {}", heuristic_name, status).as_str(),
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
        "[R] Backtracker | [P] Prim's Algo | [B] Prims Braid | [E] Eller Braids",
        text_x,
        ui_y_start + 25.0 + line_spacing * 2.0,
        20.0,
        CYAN,
    );

    draw_text(
        format!(
            "[1-{}] Change Heuris | [Space] New Maze | [A] Skip Gen",
            HEURISTIC.len()
        )
        .as_str(),
        text_x,
        ui_y_start + 25.0 + line_spacing * 3.0,
        20.0,
        GOLD,
    );
}
