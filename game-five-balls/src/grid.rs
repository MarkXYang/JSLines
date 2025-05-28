use crate::ball::{Ball, BallColor};
use rand::Rng;
use std::collections::HashSet; // For collecting unique line coordinates easily

#[derive(Debug, Clone)] // Added Clone for Solver potentially cloning it
pub struct Grid {
    pub size: usize,
    pub cells: Vec<Vec<Option<Ball>>>,
    ball_count: u32, // Used for generating unique ball IDs
}

impl Grid {
    pub fn new(size: usize) -> Self {
        Self {
            size,
            cells: vec![vec![None; size]; size],
            ball_count: 0,
        }
    }

    pub fn is_within_bounds(&self, row: usize, col: usize) -> bool {
        row < self.size && col < self.size
    }
    
    pub fn get_next_ball_id(&mut self) -> u32 {
        self.ball_count += 1;
        self.ball_count
    }

    // Helper for tests to place a ball at a specific location
    #[cfg(test)]
    fn place_ball_at(&mut self, row: usize, col: usize, color: BallColor) -> Option<u32> {
        if self.is_within_bounds(row, col) && self.cells[row][col].is_none() {
            let id = self.get_next_ball_id();
            self.cells[row][col] = Some(Ball::new(id, color));
            Some(id)
        } else {
            None
        }
    }

    pub fn place_ball_at_random_empty(&mut self, color: BallColor) -> Result<(usize, usize), &'static str> {
        let mut empty_cells = Vec::new();
        for r in 0..self.size {
            for c in 0..self.size {
                if self.cells[r][c].is_none() {
                    empty_cells.push((r, c));
                }
            }
        }

        if empty_cells.is_empty() {
            return Err("No empty cells to place a ball.");
        }

        let (row, col) = empty_cells[rand::thread_rng().gen_range(0..empty_cells.len())];
        self.cells[row][col] = Some(Ball::new(self.get_next_ball_id(), color));
        Ok((row, col))
    }

    pub fn is_full(&self) -> bool {
        self.cells.iter().all(|row| row.iter().all(Option::is_some))
    }

    pub fn check_lines(&self) -> Vec<Vec<(usize, usize)>> {
        let mut all_lines_coords = Vec::new();
        let min_line_len = 5;

        // Check Horizontal Lines
        for r in 0..self.size {
            let mut current_line = Vec::new();
            let mut current_color: Option<BallColor> = None;
            for c in 0..self.size {
                match &self.cells[r][c] {
                    Some(ball) => {
                        if Some(ball.color) == current_color {
                            current_line.push((r, c));
                        } else {
                            if current_line.len() >= min_line_len {
                                all_lines_coords.push(current_line.clone());
                            }
                            current_line = vec![(r, c)];
                            current_color = Some(ball.color);
                        }
                    }
                    None => {
                        if current_line.len() >= min_line_len {
                            all_lines_coords.push(current_line.clone());
                        }
                        current_line.clear();
                        current_color = None;
                    }
                }
            }
            if current_line.len() >= min_line_len {
                all_lines_coords.push(current_line);
            }
        }

        // Check Vertical Lines
        for c in 0..self.size {
            let mut current_line = Vec::new();
            let mut current_color: Option<BallColor> = None;
            for r in 0..self.size {
                match &self.cells[r][c] {
                    Some(ball) => {
                        if Some(ball.color) == current_color {
                            current_line.push((r, c));
                        } else {
                            if current_line.len() >= min_line_len {
                                all_lines_coords.push(current_line.clone());
                            }
                            current_line = vec![(r, c)];
                            current_color = Some(ball.color);
                        }
                    }
                    None => {
                        if current_line.len() >= min_line_len {
                            all_lines_coords.push(current_line.clone());
                        }
                        current_line.clear();
                        current_color = None;
                    }
                }
            }
            if current_line.len() >= min_line_len {
                all_lines_coords.push(current_line);
            }
        }
        
        // TODO: Add Diagonal Line Checks
        
        all_lines_coords
    }

    pub fn remove_lines(&mut self, lines_to_remove: &[Vec<(usize, usize)>]) {
        let mut coords_to_clear = HashSet::new();
        for line in lines_to_remove {
            for &(r, c) in line {
                if self.is_within_bounds(r, c) {
                    coords_to_clear.insert((r, c));
                }
            }
        }
        for (r, c) in coords_to_clear {
            self.cells[r][c] = None;
        }
    }
        
    pub fn clear_balls(&mut self, ball_coords: &Vec<(usize, usize)>) { 
        for &(row, col) in ball_coords {
            if self.is_within_bounds(row, col) {
                 self.cells[row][col] = None;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ball::BallColor; // Ensure BallColor is accessible for tests

    #[test]
    fn test_grid_new() {
        let size = 9;
        let grid = Grid::new(size);
        assert_eq!(grid.size, size);
        assert_eq!(grid.cells.len(), size);
        for row in grid.cells {
            assert_eq!(row.len(), size);
            for cell in row {
                assert!(cell.is_none());
            }
        }
        assert_eq!(grid.ball_count, 0);
    }

    #[test]
    fn test_grid_place_and_clear_ball() {
        let mut grid = Grid::new(5);
        let color = BallColor::Red;
        
        // Use the test helper to place a ball deterministically
        let ball_id_opt = grid.place_ball_at(2, 2, color);
        assert!(ball_id_opt.is_some());
        assert!(grid.cells[2][2].is_some());
        assert_eq!(grid.cells[2][2].as_ref().unwrap().color, color);

        // Clear the ball
        grid.clear_balls(&vec![(2,2)]);
        assert!(grid.cells[2][2].is_none());
    }
    
    #[test]
    fn test_grid_is_full() {
        let size = 2;
        let mut grid = Grid::new(size);
        assert!(!grid.is_full(), "Empty grid should not be full.");

        // Fill all but one cell
        grid.place_ball_at(0, 0, BallColor::Red);
        grid.place_ball_at(0, 1, BallColor::Blue);
        grid.place_ball_at(1, 0, BallColor::Green);
        assert!(!grid.is_full(), "Grid with one empty cell should not be full.");

        // Fill the last cell
        grid.place_ball_at(1, 1, BallColor::Yellow);
        assert!(grid.is_full(), "Completely filled grid should be full.");
    }

    #[test]
    fn test_grid_check_lines_basic() {
        let size = 5;
        let mut grid = Grid::new(size);
        let color = BallColor::Red;

        // Test no lines
        assert!(grid.check_lines().is_empty(), "Empty grid should have no lines.");

        // Test horizontal line of 4 (should not be detected)
        for i in 0..4 {
            grid.place_ball_at(0, i, color);
        }
        assert!(grid.check_lines().is_empty(), "Line of 4 should not be detected.");
        
        // Add one more to make it 5
        grid.place_ball_at(0, 4, color);
        let lines_h = grid.check_lines();
        assert_eq!(lines_h.len(), 1, "Horizontal line of 5 not detected. Lines: {:?}", lines_h);
        if !lines_h.is_empty() {
            assert_eq!(lines_h[0].len(), 5, "Horizontal line length is not 5.");
            assert_eq!(lines_h[0][0], (0,0));
        }


        // Clear grid for vertical test
        grid = Grid::new(size); 
        for i in 0..5 {
            grid.place_ball_at(i, 0, color);
        }
        let lines_v = grid.check_lines();
        assert_eq!(lines_v.len(), 1, "Vertical line of 5 not detected. Lines: {:?}", lines_v);
        if !lines_v.is_empty() {
            assert_eq!(lines_v[0].len(), 5, "Vertical line length is not 5.");
            assert_eq!(lines_v[0][0], (0,0));
        }
    }

    #[test]
    fn test_grid_remove_lines() {
        let size = 5;
        let mut grid = Grid::new(size);
        let color = BallColor::Blue;

        for i in 0..5 {
            grid.place_ball_at(1, i, color); // Horizontal line at row 1
        }
        
        let lines_to_remove = grid.check_lines();
        assert!(!lines_to_remove.is_empty(), "No lines found to remove, test setup error.");
        
        grid.remove_lines(&lines_to_remove);
        
        for i in 0..5 {
            assert!(grid.cells[1][i].is_none(), "Cell (1,{}) was not cleared after remove_lines.", i);
        }
    }
}
