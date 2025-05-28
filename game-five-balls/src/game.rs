use crate::ball::{Ball, BallColor};
use crate::grid::Grid;
use crate::solver::Solver;

/**
 * Diagonal line detection in Solver will be complex, similar to the JS version.
 * Game::handle_move will orchestrate pathfinding, moving balls, checking lines, and placing new balls.
 * Pathfinding (BFS) will need to be implemented.
 */
pub struct Game {
    pub grid: Grid,
    pub score: u32,
    pub upcoming_balls: Vec<BallColor>,
    // pub selected_ball_coords: Option<(usize, usize)>,
    pub solver: Solver,

    pub game_over: bool,
}
impl Game {
    pub fn new(grid_size: usize) -> Self {
        let grid = Grid::new(grid_size);
        let solver = Solver::new(grid);
        Self { grid, solver, score: 0, upcoming_balls: Vec::new(), game_over: false }
    }
    pub fn initialize_game(&mut self) {
        self.place_random_balls(10);
        self.generate_upcoming_balls();
    }
    // new, initialize_game, generate_upcoming_balls
    pub fn generate_upcoming_balls(&mut self) {
        self.upcoming_balls = vec![BallColor::random_color(); 3];
    }

    pub fn place_random_balls(&mut self, count: usize) {
        for _ in 0..count {
            let color = BallColor::random_color();
            self.grid.place_ball_at_random_empty(color).unwrap();
        }
    }

    pub fn place_upcoming_balls_on_grid(&mut self) -> Result<(), &'static str> {
        for color in self.upcoming_balls.iter() {
            self.grid.place_ball_at_random_empty(color.clone()).unwrap();
        }
        self.generate_upcoming_balls();
        Ok(())
    }

    pub fn handle_move(&mut self, start_coords: (usize, usize), end_coords: (usize, usize)) -> Result<(), &'static str> {
        // 1. find_path(...)
        // 2. If path, self.grid.move_ball_on_grid(...)
        // 3. If ball moved, lines = Solver::scan_and_collect_lines_to_clear(...)
        // 4. If lines not empty:
        //    - Use HashSet to count unique balls in lines for score
        //    - self.grid.clear_balls(...); self.score += ...
        // 5. Else (no lines from move):
        //    - self.place_upcoming_balls_on_grid() - handle error (game over?)
        //    - new_lines = Solver::scan_and_collect_lines_to_clear(...)
        //    - If new_lines not empty, clear them and score.
        // Return Ok or Err (no path, grid full)
        let path = self.solver.find_path(start_coords, end_coords);
        if path.is_some() {
            self.grid.move_ball_on_grid(start_coords, end_coords).unwrap();
        }
        let lines = self.solver.scan_and_collect_lines_to_clear();
        if lines.is_empty() {
            self.place_upcoming_balls_on_grid().unwrap();
        }  
        let new_lines = self.solver.scan_and_collect_lines_to_clear();
        if !new_lines.is_empty() {
            self.grid.clear_balls(&new_lines);
            self.score += new_lines.len() as u32;
        }
        if self.is_grid_full() {
            self.game_over = true;
        }
        if self.game_over {
            Err("Game over")
        } else {
            Ok(())
        }
    }

    pub fn is_grid_full(&self) -> bool { 
        self.grid.cells.iter().all(|row| row.iter().all(|cell| cell.is_some()))
    }
}