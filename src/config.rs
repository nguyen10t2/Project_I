use crate::constants::*;

#[derive(Clone, Debug)]
pub struct AppConfig {
    pub grid_height: usize,
    pub grid_width: usize,
    pub cell_size: f32,
    pub screen_width: f32,
    pub screen_height: f32,
    pub ui_height: f32,
    pub simulation_speed: usize,
}

impl Default for AppConfig {
    fn default() -> Self {
        let grid_height = MAZE_HEIGHT;
        let grid_width = MAZE_WIDTH;
        let cell_size = TILE_SIZE;
        let ui_height = UI_HEIGHT as f32;
        let screen_width = grid_width as f32 * cell_size;
        let screen_height = grid_height as f32 * cell_size + ui_height;

        Self {
            grid_height,
            grid_width,
            cell_size,
            screen_width,
            screen_height,
            ui_height,
            simulation_speed: STEPS_PER_FRAME,
        }
    }
}
