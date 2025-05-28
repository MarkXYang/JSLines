use crate::ball::{Ball, BallColor};
use crate::grid::Grid;
use crate::solver::Solver; // Assuming Solver and its find_path method are correctly defined

// Helper function for scoring
fn calculate_score_for_lines(lines_data: &[Vec<(usize, usize)>]) -> u32 {
    let mut score = 0;
    for line in lines_data {
        // Score: 5 for 5 balls, 10 for 6, 17 for 7, etc.
        // (length - 5) * 5 + 5 points, e.g. 5 balls = 5, 6 balls = 10
        if line.len() >= 5 {
             score += (line.len() as u32 - 5) * 5 + 5;
        }
    }
    score
}

#[derive(Debug)]
pub struct Game {
    pub grid: Grid,
    pub score: u32,
    pub upcoming_balls: Vec<BallColor>,
    pub solver: Solver,
    pub selected_ball_pos: Option<(usize, usize)>,
    pub game_over: bool,
}

impl Game {
    pub fn new(grid_size: usize) -> Self {
        let game_grid = Grid::new(grid_size);
        let solver = Solver::new(game_grid.clone()); // Solver needs a way to see the grid
        Self {
            grid: game_grid,
            solver,
            score: 0,
            upcoming_balls: Vec::new(),
            selected_ball_pos: None,
            game_over: false,
        }
    }

    pub fn initialize_game(&mut self) {
        self.add_random_balls(5); // Initial balls
        self.generate_upcoming_balls();
        self.check_game_over(); // Check if initial placement fills the board
    }

    pub fn generate_upcoming_balls(&mut self) {
        if self.game_over { return; }
        self.upcoming_balls = (0..3).map(|_| BallColor::random_color()).collect();
    }

    pub fn add_random_balls(&mut self, count: usize) {
        if self.game_over { return; }
        for _ in 0..count {
            let color = BallColor::random_color();
            if self.grid.place_ball_at_random_empty(color).is_err() {
                // Cannot place ball, grid might be full or getting full
                // self.check_game_over() will handle the full grid scenario after loop
                break; 
            }
        }
        self.check_game_over(); // Check game over after attempting to add balls
    }
    
    pub fn check_game_over(&mut self) {
        if self.grid.is_full() { // Assumes grid.is_full() is implemented
            self.game_over = true;
        }
    }

    pub fn select_cell(&mut self, row: usize, col: usize) {
        if self.game_over { return; }

        if !(row < self.grid.size && col < self.grid.size) {
            return; // Click out of bounds
        }

        if self.grid.cells[row][col].is_some() {
            // Clicked on a cell with a ball
            if self.selected_ball_pos == Some((row, col)) {
                self.selected_ball_pos = None; // Deselect if clicking the same selected ball
            } else {
                self.selected_ball_pos = Some((row, col)); // Select the ball
            }
        } else {
            // Clicked on an empty cell
            if let Some(selected_pos) = self.selected_ball_pos {
                // A ball is selected, try to move it
                // Assuming find_path takes &Grid, start_pos, end_pos
                if self.solver.find_path(&self.grid, selected_pos, (row, col)).is_some() {
                    // Path exists, move the ball
                    if let Some(ball_to_move) = self.grid.cells[selected_pos.0][selected_pos.1].take() {
                        self.grid.cells[row][col] = Some(ball_to_move);
                    }
                    self.selected_ball_pos = None; // Clear selection after move

                    // Consequence logic:
                    // 1. Check for lines formed by the moved ball.
                    let mut lines_formed = self.grid.check_lines(); // Expect Vec<Vec<(usize,usize)>>
                    
                    if !lines_formed.is_empty() {
                        // Lines were formed by the move
                        self.score += calculate_score_for_lines(&lines_formed);
                        self.grid.remove_lines(&lines_formed); // remove_lines takes &Vec<Vec<(usize,usize)>>
                    } else {
                        // No lines formed by the move itself, so add new balls.
                        self.add_random_balls(3);
                        // Check if these newly added balls formed lines.
                        lines_formed = self.grid.check_lines();
                        if !lines_formed.is_empty() {
                            self.score += calculate_score_for_lines(&lines_formed);
                            self.grid.remove_lines(&lines_formed);
                        }
                    }
                    
                    self.generate_upcoming_balls(); // Prepare next set of upcoming balls
                    self.check_game_over();       // Check if game is over
                } else {
                    // No path found, selection remains. User might try a different empty cell.
                }
            }
            // If no ball is selected and an empty cell is clicked, nothing happens.
        }
    }
}
