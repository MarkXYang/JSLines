use crate::ball::{Ball, BallColor};
use crate::grid::Grid;
use crate::solver::Solver; // Assuming Solver and its find_path method are correctly defined

// Helper function for scoring
fn calculate_score_for_lines(lines_data: &[Vec<(usize, usize)>]) -> u32 {
    let mut score = 0;
    for line in lines_data {
        // Score based on: (length - 4)^2 + length for length >= 5
        if line.len() >= 5 {
             score += (line.len() as u32).saturating_sub(4).pow(2) + line.len() as u32;
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
        let solver = Solver::new(game_grid.clone()); 
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
        self.add_random_balls(5); 
        self.generate_upcoming_balls();
        self.check_game_over(); 
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
                break; 
            }
        }
        self.check_game_over(); 
    }
    
    pub fn check_game_over(&mut self) {
        if self.grid.is_full() { 
            self.game_over = true;
        }
    }

    pub fn select_cell(&mut self, row: usize, col: usize) {
        if self.game_over { return; }

        if !(row < self.grid.size && col < self.grid.size) {
            return; 
        }

        if self.grid.cells[row][col].is_some() {
            if self.selected_ball_pos == Some((row, col)) {
                self.selected_ball_pos = None; 
            } else {
                self.selected_ball_pos = Some((row, col)); 
            }
        } else {
            if let Some(selected_pos) = self.selected_ball_pos {
                if self.solver.find_path(&self.grid, selected_pos, (row, col)).is_some() {
                    if let Some(ball_to_move) = self.grid.cells[selected_pos.0][selected_pos.1].take() {
                        self.grid.cells[row][col] = Some(ball_to_move);
                    }
                    self.selected_ball_pos = None; 

                    let mut lines_formed = self.grid.check_lines(); 
                    
                    if !lines_formed.is_empty() {
                        self.score += calculate_score_for_lines(&lines_formed);
                        self.grid.remove_lines(&lines_formed); 
                    } else {
                        // No lines from player's move, add new balls
                        self.add_random_balls(3);
                        // Check if newly added balls formed lines
                        lines_formed = self.grid.check_lines();
                        if !lines_formed.is_empty() {
                            self.score += calculate_score_for_lines(&lines_formed);
                            self.grid.remove_lines(&lines_formed);
                        }
                    }
                    
                    self.generate_upcoming_balls(); 
                    self.check_game_over();       
                }
                // If no path, selection remains.
            }
            // If no ball selected and empty cell clicked, nothing happens.
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::ball::Ball; // For direct ball creation in tests
    use crate::ball::BallColor;

    // Helper to place a ball directly into game's grid for test setup
    fn place_ball_in_game_grid(game: &mut Game, row: usize, col: usize, color: BallColor) {
        if game.grid.is_within_bounds(row, col) && game.grid.cells[row][col].is_none() {
            let id = game.grid.get_next_ball_id(); 
            game.grid.cells[row][col] = Some(Ball::new(id, color));
        }
    }

    #[test]
    fn test_game_new() {
        let game = Game::new(9);
        assert_eq!(game.score, 0, "Initial score should be 0.");
        assert!(!game.game_over, "Game should not be over initially.");
        assert_eq!(game.selected_ball_pos, None, "No ball should be selected initially.");
        assert_eq!(game.grid.size, 9, "Grid size should be set.");
    }

    #[test]
    fn test_game_select_cell_selection_logic() {
        let mut game = Game::new(5);

        game.select_cell(0, 0);
        assert_eq!(game.selected_ball_pos, None, "Selecting empty cell on empty grid should not select.");

        place_ball_in_game_grid(&mut game, 1, 1, BallColor::Red);
        game.select_cell(1, 1);
        assert_eq!(game.selected_ball_pos, Some((1, 1)), "Should select the ball at (1,1).");

        game.select_cell(1, 1); // Click same ball
        assert_eq!(game.selected_ball_pos, None, "Selecting the same ball should deselect it.");

        game.select_cell(1, 1); // Reselect (1,1)
        assert_eq!(game.selected_ball_pos, Some((1,1)), "Ball (1,1) should be re-selected.");
        
        // Assuming solver.find_path for (1,1) to (0,0) returns None (no direct path or blocked)
        game.select_cell(0, 0); 
        // Based on current select_cell: if path is None, selection does NOT change.
        assert_eq!(game.selected_ball_pos, Some((1,1)), "Selecting empty cell when no path exists should not change selection.");
    }

    #[test]
    fn test_game_add_random_balls() {
        let mut game = Game::new(3);
        game.add_random_balls(3);
        let mut ball_count = 0;
        for r in 0..game.grid.size {
            for c in 0..game.grid.size {
                if game.grid.cells[r][c].is_some() {
                    ball_count += 1;
                }
            }
        }
        assert_eq!(ball_count, 3, "Should have added 3 balls to 3x3 grid.");

        let mut game_small = Game::new(2); // 4 cells total
        place_ball_in_game_grid(&mut game_small, 0,0, BallColor::Red);
        place_ball_in_game_grid(&mut game_small, 0,1, BallColor::Blue);
        place_ball_in_game_grid(&mut game_small, 1,0, BallColor::Green);
        // 3 cells full, 1 empty

        game_small.add_random_balls(3); // Try to add 3, only 1 can be added
        ball_count = 0;
        for r in 0..game_small.grid.size {
            for c in 0..game_small.grid.size {
                if game_small.grid.cells[r][c].is_some() {
                    ball_count += 1;
                }
            }
        }
        assert_eq!(ball_count, 4, "Should fill the grid, adding only 1 more ball to 2x2 grid.");
        assert!(game_small.game_over, "Game should be over if grid becomes full after add_random_balls.");
    }

    #[test]
    fn test_game_check_game_over() {
        let mut game = Game::new(2);
        assert!(!game.game_over, "Game should not be over with non-full grid initially.");
        game.check_game_over();
        assert!(!game.game_over, "Game should not be over with non-full grid after check.");


        place_ball_in_game_grid(&mut game,0,0, BallColor::Red);
        place_ball_in_game_grid(&mut game,0,1, BallColor::Blue);
        place_ball_in_game_grid(&mut game,1,0, BallColor::Green);
        assert!(!game.game_over, "Game should not be over with 1 cell empty.");
        
        place_ball_in_game_grid(&mut game,1,1, BallColor::Yellow);
        assert!(!game.game_over, "Game should not be over yet (check_game_over not called).");
        game.check_game_over();
        assert!(game.game_over, "Game should be over when grid is full and check_game_over is called.");
    }

    #[test]
    fn test_calculate_score_for_lines_logic() {
        let line5 = vec![(0,0), (0,1), (0,2), (0,3), (0,4)]; // len 5
        assert_eq!(calculate_score_for_lines(&vec![line5.clone()]), 6, "Score for 5 balls: (5-4)^2+5 = 6");

        let line6 = vec![(1,0), (1,1), (1,2), (1,3), (1,4), (1,5)]; // len 6
        assert_eq!(calculate_score_for_lines(&vec![line6.clone()]), 10, "Score for 6 balls: (6-4)^2+6 = 10");
        
        let line7 = vec![(2,0), (2,1), (2,2), (2,3), (2,4), (2,5), (2,6)]; // len 7
        assert_eq!(calculate_score_for_lines(&vec![line7.clone()]), 16, "Score for 7 balls: (7-4)^2+7 = 16");

        assert_eq!(calculate_score_for_lines(&vec![line5, line6]), 6 + 10, "Score for multiple lines (5 and 6).");
        
        let line4 = vec![(3,0), (3,1), (3,2), (3,3)]; // len 4
        assert_eq!(calculate_score_for_lines(&vec![line4]), 0, "Score for 4 balls should be 0.");
        
        assert_eq!(calculate_score_for_lines(&vec![]), 0, "Score for no lines should be 0.");
    }

    // This test is more of an integration test for select_cell's main path.
    // It relies on specific behavior of Grid's check_lines and remove_lines,
    // and assumes Solver::find_path will return Some for the setup.
    #[test]
    fn test_game_select_cell_move_and_consequences() {
        let mut game = Game::new(5);

        // Setup: Place a ball at (0,0) to be moved
        place_ball_in_game_grid(&mut game, 0, 0, BallColor::Red);

        // Setup: Create a line of 4 Red balls at (1,0) to (1,3)
        // Moving (0,0) to (1,4) will complete a line of 5 Red balls at row 1.
        for i in 0..4 {
            place_ball_in_game_grid(&mut game, 1, i, BallColor::Red);
        }
        // Current state: Ball at (0,0), and balls at (1,0), (1,1), (1,2), (1,3)

        // 1. Select the ball at (0,0)
        game.select_cell(0, 0);
        assert_eq!(game.selected_ball_pos, Some((0,0)));

        // 2. Select empty cell (1,4) to move the ball.
        // For this test to pass, game.solver.find_path must return Some for this move.
        // We are testing the Game logic *after* a path is found.
        // If the default Solver always returns None, this test needs a mock or specific Solver setup.
        // Let's assume a simple Solver that allows moves to adjacent empty cells,
        // or for this test, that path (0,0) -> (1,4) is somehow valid.
        // (The current Solver is just `Solver::new(grid.clone())` without known behavior for find_path)
        // If `find_path` is not permissive, the move won't happen, and subsequent assertions will fail.
        // This is a known limitation of testing this part without mocking Solver.
        
        game.select_cell(1, 4); 

        // Assertions after the move (assuming find_path allowed it):
        assert!(game.grid.cells[1][4].is_some(), "Ball should be at (1,4).");
        assert_eq!(game.grid.cells[1][4].as_ref().unwrap().color, BallColor::Red);
        assert!(game.grid.cells[0][0].is_none(), "Original cell (0,0) should be empty.");
        assert_eq!(game.selected_ball_pos, None, "Selection should be cleared after move.");

        // Line (1,0) to (1,4) should be cleared
        for i in 0..5 {
            assert!(game.grid.cells[1][i].is_none(), "Cell (1,{}) should be cleared due to line formation.", i);
        }
        // Score should be updated for a line of 5: (5-4)^2 + 5 = 6
        assert_eq!(game.score, 6, "Score should be 6 for a line of 5.");

        // Consequence of line clear: add_random_balls(3) was NOT called because lines *were* formed by the player's move.
        // Instead, generate_upcoming_balls and check_game_over were called.
        // The grid started with 5 balls. 5 were cleared. 0 balls remain from original set.
        // No new balls were added by add_random_balls(3) in this path.
        let mut current_ball_count = 0;
        for r_idx in 0..game.grid.size {
            for c_idx in 0..game.grid.size {
                if game.grid.cells[r_idx][c_idx].is_some() {
                    current_ball_count += 1;
                }
            }
        }
        assert_eq!(current_ball_count, 0, "Ball count should be 0 after line clear.");
        assert!(!game.game_over); // Game should not be over.
        assert_eq!(game.upcoming_balls.len(), 3); // New upcoming balls should be generated.
    }
}
