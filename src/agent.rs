use crate::node::Node;
use macroquad::prelude::*;
use std::collections::VecDeque;

#[derive(Clone, Debug)]
pub struct Agent {
    pub position: Vec2,
    pub target: Option<Vec2>,
    pub grid_path: VecDeque<Node>,
    pub color: Color,
    pub speed: f32, // Tiles per second
    pub path_index: usize,
    pub is_main: bool,
    pub trail: Vec<Vec2>,
    pub start_position: Vec2,         // Store starting position for reset
    pub initial_target: Option<Vec2>, // Store initial target for reset
    pub heuristic_index: usize,       // Assigned heuristic index
    pub blocked_time: f32,            // Time spent blocked
}

impl Agent {
    pub fn new(start_node: Node, color: Color, is_main: bool) -> Self {
        Self {
            position: vec2(start_node.x as f32, start_node.y as f32),
            target: None,
            grid_path: VecDeque::new(),
            color,
            speed: if is_main { 4.0 } else { 3.0 }, // Slower speed
            path_index: 0,
            is_main,
            trail: Vec::new(),
            start_position: vec2(start_node.x as f32, start_node.y as f32),
            initial_target: None,
            heuristic_index: 0,
            blocked_time: 0.0,
        }
    }

    pub fn set_path(&mut self, path: Vec<Node>) {
        self.grid_path = VecDeque::from(path);
        self.path_index = 0;

        // Remove start node from path if it matches current position to avoid stutter
        if let Some(first) = self.grid_path.front() {
            if (first.x as f32 - self.position.x).abs() < 0.1
                && (first.y as f32 - self.position.y).abs() < 0.1
            {
                self.grid_path.pop_front();
            }
        }
    }

    pub fn update(&mut self, dt: f32) {
        if let Some(target) = self.target {
            let direction = target - self.position;
            let distance = direction.length();

            if distance < self.speed * dt {
                self.position = target;
                self.target = None;
            } else {
                self.position += direction.normalize() * self.speed * dt;
            }
        } else if let Some(next_node) = self.grid_path.pop_front() {
            self.target = Some(vec2(next_node.x as f32, next_node.y as f32));
        }

        // Record trail for all agents
        if self.trail.is_empty() {
            self.trail.push(self.position);
        } else if let Some(last) = self.trail.last() {
            if last.distance(self.position) > 0.5 {
                self.trail.push(self.position);
                if self.trail.len() > 150 {
                    // Limit trail length for performance/visuals
                    self.trail.remove(0);
                }
            }
        }
    }

    pub fn reset_to_start(&mut self) {
        self.position = self.start_position;
        self.target = None;
        self.grid_path.clear();
        self.trail.clear();
        self.blocked_time = 0.0;
    }

    pub fn draw(&self, cell_size: f32) {
        let center_x = self.position.x * cell_size + cell_size / 2.0;
        let center_y = self.position.y * cell_size + cell_size / 2.0;

        // Draw Trail (for all agents)
        for i in 0..self.trail.len().saturating_sub(1) {
            let p1 = self.trail[i];
            let p2 = self.trail[i + 1];
            // Fade trail based on index, but kept more visible
            let alpha = 0.8 * (i as f32 / self.trail.len() as f32);
            draw_line(
                p1.x * cell_size + cell_size / 2.0,
                p1.y * cell_size + cell_size / 2.0,
                p2.x * cell_size + cell_size / 2.0,
                p2.y * cell_size + cell_size / 2.0,
                4.0, // Thicker line
                Color::new(self.color.r, self.color.g, self.color.b, alpha),
            );
        }

        // Draw Outline
        draw_circle(center_x, center_y, cell_size / 1.5, WHITE); // Larger outline

        // Draw Main Body
        draw_circle(center_x, center_y, cell_size / 1.8, self.color); // Larger body

        if self.is_main {
            // Draw Star/Icon for Main
            draw_circle(center_x, center_y, cell_size / 3.0, YELLOW);
        }
    }
}
