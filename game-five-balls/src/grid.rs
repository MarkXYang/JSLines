use crate::ball::{Ball, BallColor}; // Ensure BallColor is imported if used directly here
use rand::Rng; // For random placement

/**
 * Grid is a 2D grid of balls.
 * It is responsible for placing balls, checking lines, and getting cells.
 */
pub struct Grid {
    pub size: usize,
    pub cells: Vec<Vec<Option<Ball>>>,
    ball_count: u32,
}

impl Grid {
    pub fn move_ball_on_grid(
        &mut self,
        start_coords: (usize, usize),
        end_coords: (usize, usize),
    ) -> Result<(), &'static str> {
        let (start_row, start_col) = start_coords;
        let (end_row, end_col) = end_coords;

        if !self.is_within_bounds(start_row, start_col) || !self.is_within_bounds(end_row, end_col) {
            return Err("Invalid coordinates");
        }
        // ... (take ball from start, place at end) ...
    }

    pub fn is_within_bounds(&self, row: usize, col: usize) -> bool { 
        row < self.size && col < self.size
     }

    pub fn clear_balls(&mut self, ball_coords: &Vec<(usize, usize)>) { 
        for (row, col) in ball_coords {
            self.cells[row][col] = None;
        }
    }

    pub fn get_next_ball_id(&mut self) -> u32 { 
        self.ball_count += 1;
        self.ball_count
    }
    pub fn place_ball_at_random_empty(&mut self, color: BallColor) -> Result<(usize, usize), &'static str> {
        let mut rng = rand::thread_rng();
        let mut empty_cells = Vec::new();

        for row in 0..self.size {
            for col in 0..self.size {
                if self.cells[row][col].is_none() {
                    empty_cells.push((row, col));
                }
            }
        }

        if empty_cells.is_empty() {
            return Err("No empty cells");
        }

        let (row, col) = empty_cells[rng.gen_range(0..empty_cells.len())];
        self.cells[row][col] = Some(Ball::new(self.get_next_ball_id(), color));
        Ok((row, col))
    }
}