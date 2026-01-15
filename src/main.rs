#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

mod agent;
mod algorithm;
mod app_state;
mod config;
mod constants;
mod generator;
mod helper;
mod heuristic;
mod maze;
mod node;
mod obstacle;

use macroquad::prelude::*;
use std::time::{Duration, Instant};

use crate::algorithm::AStarVisualizer;
use crate::app_state::{AppMode, AppState};
use crate::config::AppConfig;
use crate::constants::*;
use crate::generator::{Algorithm, MazeVisualizer};
use crate::heuristic::*;
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

#[macroquad::main(window_conf)]
async fn main() {
    let config = AppConfig::default();
    let mut app_state = AppState::new(config);

    // Initial setup
    let mut current_algo = Algorithm::RecursiveBacktracker;
    app_state.generator = Some(MazeVisualizer::new(&mut app_state.maze, current_algo));
    app_state.mode = AppMode::MazeGeneration;

    let mut current_heuristic: fn(Node, Node) -> f32 = manhattan;
    let mut heuristic_name = "Manhattan";

    let mut time_accumulator = 0f64;
    let mut start_time = Instant::now();
    let mut elapsed_duration = Duration::ZERO;
    let mut steps_count = 0;

    loop {
        clear_background(LIGHTGRAY);

        let mut reset = false;

        // Input Handling
        for (i, (key, func, name)) in HEURISTIC.iter().enumerate() {
            if is_key_pressed(*key) {
                current_heuristic = *func;
                heuristic_name = *name;

                // Mode 1: Update Solver (Classic)
                if let AppMode::Pathfinding = app_state.mode {
                    // Check if we are focusing on Solver (no agents?) or just update solver anyway
                    app_state.solver = Some(AStarVisualizer::new(&app_state.maze));
                    time_accumulator = 0.0;
                    start_time = Instant::now();
                    elapsed_duration = Duration::ZERO;
                    steps_count = 0;
                }

                // Mode 2: Reset & Replath **Main Agent ONLY**
                for agent in &mut app_state.agents {
                    if agent.is_main {
                        agent.heuristic_index = i; // Assign new heuristic
                        agent.reset_to_start();

                        // Main Agent Target (Fixed Bottom-Right)
                        let target_node = Node::new(
                            app_state.config.grid_width - 2,
                            app_state.config.grid_height - 2,
                        );
                        let start_node = Node::new(
                            agent.position.x.round() as usize,
                            agent.position.y.round() as usize,
                        );

                        if let Some(path) = AStarVisualizer::find_path(
                            &app_state.maze,
                            start_node,
                            target_node,
                            *func, // New heuristic
                            &app_state.obstacles,
                        ) {
                            agent.set_path(path);
                        }
                    }
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
            app_state.reset_maze();
            app_state.generator = Some(MazeVisualizer::new(&mut app_state.maze, current_algo));
            app_state.mode = AppMode::MazeGeneration;
            start_time = Instant::now();
            elapsed_duration = Duration::ZERO;
            steps_count = 0;
            time_accumulator = 0.0;
        }

        if is_key_pressed(KeyCode::A) {
            if let AppMode::MazeGeneration = app_state.mode {
                if let Some(generator) = &mut app_state.generator {
                    while !generator.done {
                        generator.step(&mut app_state.maze);
                    }
                }
            }
        }

        // M Key: Toggle Mode (Classic Solver <-> Agent Sim)
        if is_key_pressed(KeyCode::M) {
            app_state.show_solver = !app_state.show_solver;
        }

        // Interaction Control
        if is_mouse_button_pressed(MouseButton::Left) {
            let (mouse_x, mouse_y) = mouse_position();
            let grid_x = (mouse_x / app_state.config.cell_size) as usize;
            let grid_y = (mouse_y / app_state.config.cell_size) as usize;

            if Maze::in_bounds(
                grid_x as isize,
                grid_y as isize,
                app_state.config.grid_width,
                app_state.config.grid_height,
            ) {
                if app_state.maze.grid[grid_y][grid_x] == Tile::Path {
                    if is_key_down(KeyCode::LeftShift) || is_key_down(KeyCode::RightShift) {
                        // Set Global Target
                        app_state.global_target = Some(Node::new(grid_x, grid_y));
                        // Re-route ALL agents immediately
                        for agent in &mut app_state.agents {
                            let start_node = Node::new(
                                agent.position.x.round() as usize,
                                agent.position.y.round() as usize,
                            );
                            if let Some(path) = AStarVisualizer::find_path(
                                &app_state.maze,
                                start_node,
                                Node::new(grid_x, grid_y),
                                current_heuristic,
                                &app_state.obstacles,
                            ) {
                                agent.set_path(path);
                                agent.target = None; // Force update
                            }
                        }
                    } else {
                        // Spawn Agent
                        let mut agent =
                            crate::agent::Agent::new(Node::new(grid_x, grid_y), RED, false);

                        let target_node = if let Some(gt) = app_state.global_target {
                            gt
                        } else {
                            // Random Target
                            let mut rng = ::rand::rng();
                            use ::rand::Rng;
                            let mut t = Node::new(1, 1);
                            for _ in 0..50 {
                                let tx = rng.random_range(1..app_state.config.grid_width - 1);
                                let ty = rng.random_range(1..app_state.config.grid_height - 1);
                                if app_state.maze.grid[ty][tx] == Tile::Path {
                                    t = Node::new(tx, ty);
                                    break;
                                }
                            }
                            t
                        };

                        if let Some(path) = AStarVisualizer::find_path(
                            &app_state.maze,
                            Node::new(grid_x, grid_y),
                            target_node,
                            // Assign Random Heuristic for Crowd
                            {
                                let mut rng = ::rand::rng();
                                use ::rand::Rng;
                                let h_idx = rng.random_range(0..HEURISTIC.len());
                                agent.heuristic_index = h_idx;
                                HEURISTIC[h_idx].1
                            },
                            &app_state.obstacles,
                        ) {
                            agent.set_path(path);
                            agent.initial_target =
                                Some(vec2(target_node.x as f32, target_node.y as f32)); // Save for reset
                            app_state.agents.push(agent);
                        }
                    }
                }
            }
        }

        // Obstacle Spawning Control (Right Click)
        if is_mouse_button_pressed(MouseButton::Right) {
            let (mouse_x, mouse_y) = mouse_position();
            let grid_x = (mouse_x / app_state.config.cell_size) as usize;
            let grid_y = (mouse_y / app_state.config.cell_size) as usize;

            if Maze::in_bounds(
                grid_x as isize,
                grid_y as isize,
                app_state.config.grid_width,
                app_state.config.grid_height,
            ) {
                if app_state.maze.grid[grid_y][grid_x] == Tile::Path {
                    app_state
                        .obstacles
                        .push(crate::obstacle::DynamicObstacle::new(Node::new(
                            grid_x, grid_y,
                        )));
                }
            }
        }
        // Main Agent Spawning (Middle Click) - Fixed Scenario (TL -> BR)
        if is_mouse_button_pressed(MouseButton::Middle) {
            // Force Start at Top-Left (1, 1)
            let start_grid = Node::new(1, 1);
            // Force Goal at Bottom-Right
            let goal_grid = Node::new(
                app_state.config.grid_width - 2,
                app_state.config.grid_height - 2,
            );

            let mut agent = crate::agent::Agent::new(start_grid, BLUE, true);

            if let Some(path) = AStarVisualizer::find_path(
                &app_state.maze,
                start_grid,
                goal_grid,
                current_heuristic,
                &app_state.obstacles,
            ) {
                agent.set_path(path);
                agent.initial_target = Some(vec2(goal_grid.x as f32, goal_grid.y as f32));
                // Set fixed start position for reset
                agent.start_position = vec2(start_grid.x as f32, start_grid.y as f32);
                app_state.agents.push(agent);
            }
        }

        // Logic Updates
        match app_state.mode {
            AppMode::MazeGeneration => {
                if let Some(generator) = &mut app_state.generator {
                    if !generator.done {
                        elapsed_duration = start_time.elapsed();
                        let delay = STEP_DELAY_SEC;

                        if delay <= 0.0001 {
                            for _ in 0..MAZE_GEN_STEPS_PER_FRAME {
                                generator.step(&mut app_state.maze);
                                steps_count += 1;
                                if generator.done {
                                    break;
                                }
                            }
                            time_accumulator = 0.0;
                        } else {
                            time_accumulator += get_frame_time() as f64;
                            if time_accumulator >= delay {
                                generator.step(&mut app_state.maze);
                                steps_count += 1;
                                time_accumulator -= delay;
                            }
                        }
                    }

                    if generator.done {
                        app_state.mode = AppMode::Pathfinding;
                        // app_state.solver = Some(AStarVisualizer::new(&app_state.maze)); // Disable auto solver for now, let agents roam
                        start_time = Instant::now();
                        elapsed_duration = Duration::ZERO;
                        steps_count = 0;
                    }
                }
            }
            AppMode::Pathfinding | AppMode::Idle => {
                let dt = get_frame_time();

                // Update Obstacles
                for obstacle in &mut app_state.obstacles {
                    obstacle.update(dt as f64, &app_state.maze);
                }

                // Collect positions of all agents for avoidance logic
                let agent_positions: Vec<Node> = app_state
                    .agents
                    .iter()
                    .map(|a| {
                        Node::new(a.position.x.round() as usize, a.position.y.round() as usize)
                    })
                    .collect();

                // Update Agents with Avoidance
                for i in 0..app_state.agents.len() {
                    let mut blocked = false;
                    let mut repath_needed = false;

                    {
                        let agent = &app_state.agents[i];
                        // If agent is stationary (no target) and wants to move (has path)
                        // It effectively "waits" at the current node if the NEXT node is occupied
                        if agent.target.is_none() {
                            if let Some(next_node) = agent.grid_path.front() {
                                let next_pos = vec2(next_node.x as f32, next_node.y as f32);
                                // Check collision with other agents
                                for (j, other) in app_state.agents.iter().enumerate() {
                                    if i != j {
                                        // If other agent is at the target cell or moving to it
                                        // Simple distance check covers both roughly
                                        if other.position.distance(next_pos) < 0.9 {
                                            blocked = true;
                                            if agent.is_main {
                                                repath_needed = true;
                                            }
                                            break;
                                        }
                                    }
                                }
                            }
                        }
                    }

                    if blocked {
                        let agent = &mut app_state.agents[i];
                        agent.blocked_time += dt;

                        // Main Agent "Dodge" Logic (Immediate)
                        if repath_needed {
                            // Try to find a detour around the blockage
                            let target_node = if let Some(t) = agent.initial_target {
                                Node::new(t.x as usize, t.y as usize)
                            } else {
                                Node::new(
                                    app_state.config.grid_width - 2,
                                    app_state.config.grid_height - 2,
                                )
                            };

                            let start_node =
                                Node::new(agent.position.x as usize, agent.position.y as usize);
                            let h_func = HEURISTIC
                                .get(agent.heuristic_index)
                                .map(|x| x.1)
                                .unwrap_or(manhattan);

                            // Build obstacles including ALL agents (Snapshot)
                            let mut loop_obstacles = app_state.obstacles.clone();
                            for (idx, pos) in agent_positions.iter().enumerate() {
                                if idx != i {
                                    loop_obstacles
                                        .push(crate::obstacle::DynamicObstacle::new(*pos));
                                }
                            }

                            if let Some(path) = AStarVisualizer::find_path(
                                &app_state.maze,
                                start_node,
                                target_node,
                                h_func,
                                &loop_obstacles,
                            ) {
                                agent.set_path(path);
                            }
                        }
                        // Crowd Tolerance Logic (Slow give-up)
                        else if !agent.is_main && agent.blocked_time > 2.0 {
                            // Too long! Pick a new random target nearby or just anywhere
                            agent.blocked_time = 0.0;

                            // Random New Target
                            let mut rng = ::rand::rng();
                            use ::rand::Rng;
                            let mut t = Node::new(1, 1);
                            for _ in 0..20 {
                                let tx = rng.random_range(1..app_state.config.grid_width - 1);
                                let ty = rng.random_range(1..app_state.config.grid_height - 1);
                                if app_state.maze.grid[ty][tx] == Tile::Path {
                                    t = Node::new(tx, ty);
                                    break;
                                }
                            }

                            let start_node =
                                Node::new(agent.position.x as usize, agent.position.y as usize);
                            let h_func = HEURISTIC
                                .get(agent.heuristic_index)
                                .map(|x| x.1)
                                .unwrap_or(manhattan);

                            if let Some(path) = AStarVisualizer::find_path(
                                &app_state.maze,
                                start_node,
                                t,
                                h_func,
                                &app_state.obstacles,
                            ) {
                                agent.set_path(path); // Go somewhere else!
                            }
                        }
                    } else {
                        app_state.agents[i].blocked_time = 0.0; // Reset patience if moving
                        app_state.agents[i].update(dt * 10.0);
                    }

                    let agent = &mut app_state.agents[i];

                    // Basic collision with obstacles
                    for obs in &app_state.obstacles {
                        if agent
                            .position
                            .distance(vec2(obs.position.x as f32, obs.position.y as f32))
                            < 1.0
                        {
                            agent.target = None;
                            agent.grid_path.clear();
                        }
                    }

                    // If agent reached target (no more path), give new target
                    if agent.target.is_none() && agent.grid_path.is_empty() {
                        let target_node = if agent.is_main {
                            // Main Agent logic: Keep going to Goal (W-2, H-2) unless Global overrides
                            if let Some(t) = agent.initial_target {
                                Node::new(t.x as usize, t.y as usize)
                            } else {
                                Node::new(
                                    app_state.config.grid_width - 2,
                                    app_state.config.grid_height - 2,
                                )
                            }
                        } else if let Some(gt) = app_state.global_target {
                            // ... existing Global Target logic ...
                            let dist = (agent.position.x - gt.x as f32).abs()
                                + (agent.position.y - gt.y as f32).abs();
                            if dist < 1.0 {
                                continue; // Finished
                            }
                            gt
                        } else {
                            // Random Logic (Wander) for Spawn Agents
                            let mut rng = ::rand::rng();
                            use ::rand::Rng;
                            let mut t = Node::new(1, 1);
                            for _ in 0..20 {
                                let tx = rng.random_range(1..app_state.config.grid_width - 1);
                                let ty = rng.random_range(1..app_state.config.grid_height - 1);
                                if app_state.maze.grid[ty][tx] == Tile::Path {
                                    t = Node::new(tx, ty);
                                    break;
                                }
                            }
                            t
                        };

                        // Main Agent: If reached goal, stop (don't re-path to same goal continuously)
                        if agent.is_main {
                            let dist = agent
                                .position
                                .distance(vec2(target_node.x as f32, target_node.y as f32));
                            if dist < 1.5 {
                                continue; // Arrived.
                            }
                        }

                        // Determine Heuristic
                        let h_func = HEURISTIC
                            .get(agent.heuristic_index)
                            .map(|x| x.1)
                            .unwrap_or(manhattan);

                        // Build Extended Obstacles for Main Agent
                        let mut loop_obstacles = app_state.obstacles.clone(); // Base obstacles
                        if agent.is_main {
                            // Add Crowd as obstacles
                            for (idx, pos) in agent_positions.iter().enumerate() {
                                if idx != i {
                                    // Don't block self
                                    loop_obstacles
                                        .push(crate::obstacle::DynamicObstacle::new(*pos));
                                }
                            }
                        }

                        let start_node =
                            Node::new(agent.position.x as usize, agent.position.y as usize);
                        if let Some(path) = AStarVisualizer::find_path(
                            &app_state.maze,
                            start_node,
                            target_node,
                            h_func,
                            &loop_obstacles,
                        ) {
                            agent.set_path(path);
                        }
                    }
                }

                if app_state.show_solver {
                    if let Some(solver) = &mut app_state.solver {
                        if !solver.found {
                            elapsed_duration = start_time.elapsed();
                            let delay = STEP_DELAY_SEC;

                            if delay <= 0.0001 {
                                time_accumulator += get_frame_time() as f64;
                                let steps = app_state.config.simulation_speed;

                                for _ in 0..steps {
                                    solver.step(&app_state.maze, current_heuristic);
                                    steps_count += 1;
                                    time_accumulator = 0.0;
                                    if solver.found {
                                        elapsed_duration = start_time.elapsed();
                                        break;
                                    }
                                }
                            } else {
                                time_accumulator += get_frame_time() as f64;
                                if time_accumulator >= delay {
                                    solver.step(&app_state.maze, current_heuristic);
                                    steps_count += 1;
                                    time_accumulator -= delay;
                                }
                            }
                        }
                    }
                }
            }
        }

        // Drawing
        app_state.maze.draw(app_state.config.cell_size);

        if app_state.show_solver {
            if let Some(solver) = &app_state.solver {
                solver.draw(&app_state.maze, app_state.config.cell_size);
            }
        }

        for obstacle in &app_state.obstacles {
            obstacle.draw(app_state.config.cell_size);
        }

        for agent in &app_state.agents {
            agent.draw(app_state.config.cell_size);
        }

        if let Some(gt) = app_state.global_target {
            let cz = app_state.config.cell_size;
            draw_rectangle(gt.x as f32 * cz, gt.y as f32 * cz, cz, cz, GOLD);
            draw_circle(
                gt.x as f32 * cz + cz / 2.0,
                gt.y as f32 * cz + cz / 2.0,
                cz / 1.5,
                RED,
            );
        } else {
            // Draw Fixed Goal (Bottom Right) if Main Agent exists
            if app_state.agents.iter().any(|a| a.is_main) {
                let gx = app_state.config.grid_width - 2;
                let gy = app_state.config.grid_height - 2;
                let cz = app_state.config.cell_size;
                draw_rectangle(
                    gx as f32 * cz,
                    gy as f32 * cz,
                    cz,
                    cz,
                    Color::new(0.0, 0.0, 0.5, 0.5),
                ); // Faint Blue
                draw_circle(
                    gx as f32 * cz + cz / 2.0,
                    gy as f32 * cz + cz / 2.0,
                    cz / 3.0,
                    BLUE,
                );
            }
        }

        let found = app_state.solver.as_ref().map_or(false, |v| v.found);
        let distance = app_state
            .solver
            .as_ref()
            .map_or(0, |v| v.path.as_ref().map_or(0, |p| p.len()));

        draw_dashboard(
            &app_state.config,
            heuristic_name,
            &app_state,
            elapsed_duration,
            steps_count,
            distance,
            found,
        );

        next_frame().await;
    }
}

fn draw_dashboard(
    config: &AppConfig,
    heuristic_name: &str,
    app_state: &AppState,
    elapsed_duration: std::time::Duration,
    steps_count: usize,
    distance: usize,
    found: bool,
) {
    let ui_y_start = config.grid_height as f32 * config.cell_size;
    let ui_height = config.ui_height as f32;
    let screen_width = config.screen_width;

    // Background
    draw_rectangle(
        0.0,
        ui_y_start,
        screen_width,
        ui_height,
        Color::new(0.05, 0.05, 0.1, 1.0), // Darker Blue-ish Gray
    );

    let text_x = 20.0;
    let mut current_y = ui_y_start + 30.0;
    let line_height = 25.0;

    // -- ROW 1: General Stats --
    let fps = get_fps();
    let agent_count = app_state.agents.len();
    let obs_count = app_state.obstacles.len();

    draw_text(
        format!(
            "FPS: {} | Agents: {} | Obstacles: {}",
            fps, agent_count, obs_count
        )
        .as_str(),
        text_x,
        current_y,
        22.0,
        WHITE,
    );
    current_y += line_height;

    // -- ROW 2: Heuristic & Mode --
    let mode_text = if app_state.config.simulation_speed > 15 {
        "Fast"
    } else {
        "Normal"
    };
    let solver_status = if app_state.show_solver {
        "Vis: ON"
    } else {
        "Vis: OFF"
    };

    draw_text(
        format!(
            "Algo: {} | Speed: {} | {}",
            heuristic_name, mode_text, solver_status
        )
        .as_str(),
        text_x,
        current_y,
        22.0,
        LIGHTGRAY,
    );
    current_y += line_height;

    // -- ROW 3: Stats --
    if app_state.show_solver {
        let status_color = if found { GREEN } else { LIGHTGRAY };
        draw_text(
            format!(
                "Solver: {:.4}s | Steps: {} | Dist: {}",
                elapsed_duration.as_secs_f32(),
                steps_count,
                distance,
            )
            .as_str(),
            text_x,
            current_y,
            22.0,
            status_color,
        );
    } else {
        // Find Main Agent stats
        let main_agent_stats = app_state.agents.iter().find(|a| a.is_main).map(|a| {
            let dist_to_target = if let Some(t) = a.target.or(a.initial_target) {
                t.distance(a.position)
            } else {
                0.0
            };
            format!(
                "Main Agent: Steps Taken ~{} | Dist to Goal: {:.1}",
                a.trail.len(),
                dist_to_target
            )
        });

        if let Some(stats) = main_agent_stats {
            draw_text(&stats, text_x, current_y, 22.0, YELLOW);
        } else {
            draw_text(
                "Spawn Main Agent [M-Click] for Stats",
                text_x,
                current_y,
                22.0,
                GRAY,
            );
        }
    }
    current_y += line_height * 1.5;

    // -- ROW 4: Controls --
    // Column 1
    draw_text("[1-6] Algorithm", text_x, current_y, 20.0, CYAN);
    draw_text("[Space] New Maze", text_x + 160.0, current_y, 20.0, CYAN);
    draw_text("[M] Toggle Mode", text_x + 340.0, current_y, 20.0, CYAN);

    current_y += line_height;
    // Column 2
    draw_text("[L-Click] Spawn Agent", text_x, current_y, 20.0, ORANGE);
    draw_text(
        "[Shift+L] Global Target",
        text_x + 240.0,
        current_y,
        20.0,
        ORANGE,
    );

    current_y += line_height;
    // Column 3
    draw_text("[R-Click] Spawn Obs", text_x, current_y, 20.0, ORANGE);
    draw_text(
        "[Mid-Click] Main Agent",
        text_x + 240.0,
        current_y,
        20.0,
        ORANGE,
    );
}
