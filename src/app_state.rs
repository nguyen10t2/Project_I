use crate::algorithm::AStarVisualizer;
use crate::config::AppConfig;
use crate::generator::Algorithm;
use crate::generator::MazeVisualizer;
use crate::maze::Maze;
use crate::node::Node;

pub enum AppMode {
    MazeGeneration,
    Pathfinding,
    Idle,
}

pub struct AppState {
    pub maze: Maze,
    pub generator: Option<MazeVisualizer>,
    pub solver: Option<AStarVisualizer>,
    pub agents: Vec<crate::agent::Agent>,
    pub obstacles: Vec<crate::obstacle::DynamicObstacle>,
    pub global_target: Option<Node>,
    pub show_solver: bool,
    pub mode: AppMode,
    pub config: AppConfig,
}

impl AppState {
    pub fn new(config: AppConfig) -> Self {
        let maze = Maze::new(config.grid_width, config.grid_height);

        Self {
            maze,
            generator: None,
            solver: None,
            agents: Vec::new(),
            obstacles: Vec::new(),
            global_target: None,
            show_solver: true,
            mode: AppMode::Idle,
            config,
        }
    }

    pub fn reset_maze(&mut self) {
        self.maze = Maze::new(self.config.grid_width, self.config.grid_height);
        self.generator = None;
        self.solver = None;
        self.agents.clear();
        self.obstacles.clear();
        self.global_target = None;
        self.show_solver = true;
        self.mode = AppMode::Idle;
    }
}
