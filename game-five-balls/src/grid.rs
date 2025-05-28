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
    
    // Removed move_ball_on_grid as game.rs handles cell manipulation now.
    
    pub fn clear_balls(&mut self, ball_coords: &Vec<(usize, usize)>) { 
        for &(row, col) in ball_coords {
            if self.is_within_bounds(row, col) {
                 self.cells[row][col] = None;
            }
        }
    }
}
