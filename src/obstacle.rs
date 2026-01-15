use crate::config::AppConfig;
use crate::maze::{Maze, Tile};
use crate::node::Node;
use macroquad::prelude::*;

#[derive(Clone, Debug)]
pub struct DynamicObstacle {
    pub position: Node,
    pub color: Color,
    // Movement logic can be expanded. For now, let's say they toggle or move linearly.
    pub move_dir: (isize, isize),
    pub move_timer: f64,
    pub move_interval: f64,
}

impl DynamicObstacle {
    pub fn new(start: Node) -> Self {
        let mut rng = ::rand::rng();
        use ::rand::Rng;
        let dirs = [(0, 1), (0, -1), (1, 0), (-1, 0)];
        let dir = dirs[rng.random_range(0..4)];

        Self {
            position: start,
            color: ORANGE,
            move_dir: dir,
            move_timer: 0.0,
            move_interval: 1.0, // Move every 1 second
        }
    }

    pub fn update(&mut self, dt: f64, maze: &Maze) {
        self.move_timer += dt;
        if self.move_timer >= self.move_interval {
            self.move_timer = 0.0;

            let nx = self.position.x as isize + self.move_dir.0;
            let ny = self.position.y as isize + self.move_dir.1;

            if Maze::in_bounds(nx, ny, maze.width, maze.height)
                && maze.grid[ny as usize][nx as usize] == Tile::Path
            {
                self.position.x = nx as usize;
                self.position.y = ny as usize;
            } else {
                // Bounce
                self.move_dir.0 *= -1;
                self.move_dir.1 *= -1;
            }
        }
    }

    pub fn draw(&self, cell_size: f32) {
        draw_rectangle(
            self.position.x as f32 * cell_size,
            self.position.y as f32 * cell_size,
            cell_size,
            cell_size,
            self.color,
        );
        // Draw an X or specific marking
        draw_line(
            self.position.x as f32 * cell_size,
            self.position.y as f32 * cell_size,
            (self.position.x as f32 + 1.0) * cell_size,
            (self.position.y as f32 + 1.0) * cell_size,
            2.0,
            BLACK,
        );
        draw_line(
            (self.position.x as f32 + 1.0) * cell_size,
            self.position.y as f32 * cell_size,
            self.position.x as f32 * cell_size,
            (self.position.y as f32 + 1.0) * cell_size,
            2.0,
            BLACK,
        );
    }
}
